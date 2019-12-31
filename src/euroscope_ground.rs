use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use fnv::FnvHashMap;
use osmpbf::*;

use crate::types::{Coords, LatLon, Refs};
use crate::types;
use crate::utils::format_coordinate;

pub(crate) fn generate_ese_ground_taxiway<P: AsRef<Path> + Copy>(source_path: P, airport_name: &str) {
    let out_path_string = format!("./out/{}.ese", airport_name);
    let out_path = Path::new(&out_path_string);
    if !out_path.parent().unwrap().exists() {
        fs::create_dir_all(out_path.parent().unwrap()).expect("unable to generate dir");
    }
    let out_file = &mut File::create(out_path).expect("unable to open file");

    out_file
        .write_all(b"[GROUND]\n")
        .expect("failed to write file");
    let taxiways = get_taxiways(source_path);

    let coords = get_taxiways_coords(source_path, taxiways);
    write_taxiways(out_file, airport_name, coords);

    out_file.sync_all().expect("unable to sync to file");
}

fn write_taxiways(out_file: &mut File, airport_name: &str, taxiways: FnvHashMap<String, Coords>) {
    taxiways.iter().for_each(|(name, coordinates)| {
        let type_string = format!("TAXI:{} {}:20:1\n", airport_name, name);
        out_file
            .write_all(type_string.as_bytes())
            .expect("failed to write file");

        let local_coords: Vec<&LatLon> = coordinates.iter().clone().collect();
        local_coords.iter().for_each(|latlon| {
            let lat_str = format_coordinate(latlon.lat);

            let lon_str = format_coordinate(latlon.lon);

            let coordinate_string: String;

            coordinate_string = format!("COORD:N{}:E{}\n", lat_str, lon_str);
            out_file
                .write_all(coordinate_string.as_bytes())
                .expect("failed to write file");
        });
    });
}

fn get_taxiways_coords<P: AsRef<Path> + Copy>(
    path: P,
    refs: FnvHashMap<String, Refs>,
) -> FnvHashMap<String, Coords> {
    let mut taxiways: FnvHashMap<String, Coords> = FnvHashMap::default();
    let mut nodes: Vec<types::Node> = Vec::new();

    let reader = ElementReader::from_path(path).unwrap();
    reader
        .for_each(|element| {
            if let Element::DenseNode(node) = element {
                let new_node = types::Node {
                    id: node.id,
                    lat: node.lat(),
                    lon: node.lon(),
                };
                nodes.push(new_node);
            }
        })
        .unwrap();

    for (key, val) in refs.iter() {
        val.iter().for_each(|refval| {
            if let Some(node) = nodes.iter().find(|x| x.id == *refval) {
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
        })
    }

    taxiways
}

fn get_taxiways<P: AsRef<Path> + Copy>(path: P) -> FnvHashMap<String, Refs> {
    let mut taxiways: FnvHashMap<String, Refs> = FnvHashMap::default();

    let reader = ElementReader::from_path(path).unwrap();
    reader
        .for_each(|element| {
            if let Element::Way(way) = element {
                if way
                    .tags()
                    .any(|(key, value)| key == "aeroway" && value == "taxiway")
                {
                    if let Some((_, value)) = way.tags().find(|(key, _)| *key == "ref") {
                        let mut new_vals = way.refs().collect::<Vec<i64>>();

                        if taxiways.contains_key(value) {
                            taxiways.get_mut(value).unwrap().append(&mut new_vals);
                        } else {
                            taxiways.insert(value.to_string(), new_vals);
                        }
                    }
                }
            }
        })
        .unwrap();
    taxiways
}
