use std::fs::File;

use osm_xml::{Reference, Tag, OSM, Way};

fn main() {
    let f = File::open("./data/EDDH.osm").unwrap();
    let doc = OSM::parse(f).unwrap();

    let way_info = way_reference_statistics(&doc);

    println!(
        "Way count {}",
        doc.ways
            .values()
            .filter(|way| {
                way.tags.contains(&Tag {
                    key: "aeroway".to_string(),
                    val: "taxiway".to_string(),
                })
            })
            .collect::<Vec<&Way>>()
            .len()
    );
    println!(
        "Way reference count: {}, invalid references: {}",
        way_info.0, way_info.1
    );
}

fn way_reference_statistics(doc: &OSM) -> (usize, usize) {
    doc.ways
        .values()
        .filter(|way| {
            way.tags.contains(&Tag {
                key: "aeroway".to_string(),
                val: "taxiway".to_string(),
            })
        })
        .flat_map(|way| way.nodes.iter())
        .fold((0, 0), |acc, node| match doc.resolve_reference(&node) {
            Reference::Node(_) => (acc.0 + 1, acc.1),
            Reference::Unresolved | Reference::Way(_) | Reference::Relation(_) => {
                (acc.0, acc.1 + 1)
            }
        })
}
