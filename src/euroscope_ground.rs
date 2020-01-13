use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use fnv::FnvHashMap;
use osmpbfreader::{groups, primitive_block_from_blob};

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

    let file = File::open(source_path).expect("unable to open file");
    let buf_reader = BufReader::new(file);
    let mut pbf = osmpbfreader::OsmPbfReader::new(buf_reader);

    for block in pbf.blobs().map(|b| primitive_block_from_blob(&b.unwrap())) {
        let block: osmpbfreader::osmformat::PrimitiveBlock = block.unwrap();

        let mut nodes: Vec<types::Node> = Vec::new();
        let mut ways: FnvHashMap<String, Refs> = FnvHashMap::default();
        let mut taxiways: FnvHashMap<String, Coords> = FnvHashMap::default();
        for group in block.get_primitivegroup().iter() {
            parse_taxiway!(group, block, ways);
            //println!("{:?}", ways);
            parse_taxiway_nodes!(group, block, ways, nodes, taxiways);
        }
        println!("{:?}", taxiways);
        write_taxiways(out_file, airport_name, taxiways);

        out_file.sync_all().expect("unable to sync to file");
    }
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
