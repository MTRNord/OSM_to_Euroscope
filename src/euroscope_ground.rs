use std::fs;
use std::fs::File;
use std::io::{BufReader, Cursor, Write};
use std::path::Path;

use fnv::FnvHashMap;
use osmio::obj_types::StringOSMObj;
use osmio::xml::XMLReader;
use osmio::{Node, OSMObjBase, OSMReader, Way};
use reqwest::{Request, Response};

use crate::types;
use crate::types::{Coords, LatLon, Refs};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

pub(crate) async fn generate_ese_ground_taxiway(
    airport_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let out_path_string = format!("./out/{}.ese", airport_name);
    let out_path = Path::new(&out_path_string);
    if !out_path.parent().unwrap().exists() {
        fs::create_dir_all(out_path.parent().unwrap()).expect("unable to generate dir");
    }
    let out_file = &mut File::create(out_path).expect("unable to open file");

    out_file
        .write_all(b"[GROUND]\n")
        .expect("failed to write file");

    let client = reqwest::Client::builder().build()?;
    let query0 = "data=";
    let query1 = "[out:xml][timeout:25];
area[icao=";
    let query2 = "]->.searchArea;
(
  node[\"aeroway\"=\"taxiway\"][\"ref\"](area.searchArea);
  way[\"aeroway\"=\"taxiway\"][\"ref\"](area.searchArea);
  relation[\"aeroway\"=\"taxiway\"][\"ref\"](area.searchArea);
);
out body;
>;
out skel qt;";
    let complete: String = [query1, airport_name, query2].concat();
    let query = percent_encode(complete.replace("\n", "").as_bytes(), NON_ALPHANUMERIC).to_string();
    let query_complete: String = [query0, query.as_str()].concat();
    let req: Request = client.post("https://overpass-api.de/api/interpreter")
        .body(query_complete).build()?;

    let resp: Response = client.execute(req).await?;

    let data = resp.text().await?;

    let data_buffer = Cursor::new(data);
    let buf_reader = BufReader::new(data_buffer);

    let mut osm = XMLReader::new(buf_reader);

    let mut nodes: Vec<types::Node> = Vec::new();
    let mut ways: FnvHashMap<String, Refs> = FnvHashMap::default();
    let mut taxiways: FnvHashMap<String, Coords> = FnvHashMap::default();

    osm.objects().for_each(|object| match object {
        StringOSMObj::Node(n) => {
            let lat_lon = n.lat_lon().expect("unable to get lat_lon");
            let new_node = types::Node {
                id: n.id(),
                lat: lat_lon.0 as f64,
                lon: lat_lon.1 as f64,
            };
            nodes.push(new_node);
        }
        StringOSMObj::Way(w) => {
            let mut tags = w.tags();
            if tags.any(|x| x.0 == "aeroway" && x.1 == "taxiway") {
                if tags.any(|x| x.0 == "ref") {
                    let value = w
                        .tags()
                        .find(|x| x.0 == "ref")
                        .expect("unable to find ref")
                        .1;
                    let mut nodes: Vec<i64> = Vec::from(w.nodes());

                    if ways.contains_key(value) {
                        ways.get_mut(value).unwrap().append(&mut nodes);
                    } else {
                        ways.insert(value.to_string(), nodes);
                    }
                }
            }
        }
        StringOSMObj::Relation(_) => {}
    });
    for (key, val) in ways.iter() {
        val.iter().for_each(|&refval| {
            if let Some(node) = nodes.iter().find(|&x| x.id == refval) {
                let lat_lon = LatLon {
                    lat: node.lat,
                    lon: node.lon,
                };

                if taxiways.contains_key(key.as_str()) {
                    (*(taxiways.get_mut(key.as_str()).unwrap())).push(lat_lon);
                } else {
                    let mut coords: Coords = Vec::new();
                    coords.push(lat_lon);
                    taxiways.insert(key.clone(), coords);
                }
            }
        });
    }

    write_taxiways(out_file, airport_name, taxiways);

    out_file.sync_all().expect("unable to sync to file");
    Ok(())
}

fn write_taxiways(out_file: &mut File, airport_name: &str, taxiways: FnvHashMap<String, Coords>) {
    taxiways.iter().for_each(|(name, coordinates)| {
        let type_string = format!("TAXI:{} {}:20:1\n", airport_name, name);
        out_file
            .write_all(type_string.as_bytes())
            .expect("failed to write file");

        let local_coords: Vec<&LatLon> = coordinates.iter().clone().collect();
        local_coords.iter().for_each(|latlon| {
            out_file
                .write_all(format_coordinate!(latlon.lat, latlon.lon))
                .expect("failed to write file");
        });
    });
}
