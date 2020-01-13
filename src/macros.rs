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

macro_rules! format_coordinate_part {
    ($coordinate:expr) => {{
        use crate::utils::to_2_digits;

        let degree = $coordinate.trunc() as i64;
        let minute = (($coordinate - degree as f64).abs() * 60.0).trunc() as i64;
        let seconds = 3600.0 * ($coordinate - degree as f64).abs() - 60.0 * (minute as f64);
        let decimal_part_string = format!("{:.2}", seconds - (seconds.trunc() as f64) as f64);
        let decimal_part_split = decimal_part_string.split('.').collect::<Vec<&str>>();
        let decimal_part = decimal_part_split.last().unwrap();

        format!(
            "{:03}.{:02}.{:02}.{:02}",
            degree,
            minute,
            to_2_digits(seconds as i32),
            decimal_part
        )
    }};
}

macro_rules! format_coordinate {
    ($lat:expr, $lon:expr) => {{
        let lat_str = format_coordinate_part!($lat);
        let lon_str = format_coordinate_part!($lon);
        format!("COORD:N{}:E{}\n", lat_str, lon_str).as_bytes()
    }};
}
