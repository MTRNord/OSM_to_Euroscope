macro_rules! parse_taxiway {
    ($group:expr, $block:expr, $taxiways:expr) => {{
        use osmpbfreader::groups;

        for way in groups::ways(&$group, &$block) {
            if way.tags.contains("aeroway", "taxiway") {
                if way.tags.contains_key("ref") {
                    let value = way.tags.get("ref").unwrap();
                    let mut nodes: Vec<i64> = way.nodes.iter().map(|x| {
                        x.0
                    }).collect();

                    if $taxiways.contains_key(value) {
                        $taxiways.get_mut(value).unwrap().append(&mut nodes);
                    } else {
                        $taxiways.insert(value.to_string(), nodes);
                    }
                }
            }
        }
    }};
}

macro_rules! parse_taxiway_nodes {
    ($group:expr, $block:expr, $needed_nodes:expr, $nodes:expr, $taxiways:expr) => {{
        use crate::types::LatLon;


        for node in groups::simple_nodes(&$group, &$block) {
            let new_node = types::Node {
                id: node.id.0,
                lat: node.lat(),
                lon: node.lon(),
            };
            $nodes.push(new_node);
        }
        for node in groups::dense_nodes(&$group, &$block) {
            let new_node = types::Node {
                id: node.id.0,
                lat: node.lat(),
                lon: node.lon(),
            };
            $nodes.push(new_node);
        }

        for (key, val) in $needed_nodes.iter() {
            val.iter().for_each(|&refval| {
                if let Some(node) = $nodes.iter().find(|&x| {x.id == refval}) {
                    let lat_lon = LatLon {
                        lat: node.lat,
                        lon: node.lon,
                    };

                    if $taxiways.contains_key(key.as_str()) {
                        (*($taxiways.get_mut(key.as_str()).unwrap())).push(lat_lon);
                    } else {
                        let mut coords: Coords = Vec::new();
                        coords.push(lat_lon);
                        $taxiways.insert(key.clone(), coords);
                    }
                }

            });

        }
    }};
}
