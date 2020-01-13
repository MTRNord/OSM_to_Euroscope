//#![feature(test)]
#![allow(non_snake_case)]

extern crate clap;

use std::process::exit;

use clap::{App, Arg, SubCommand};

use crate::euroscope_ground::generate_ese_ground_taxiway;

mod types;
mod utils;
#[macro_use]
mod macros;
mod euroscope_ground;

fn main() {
    let matches = App::new("OSM_to_Euroscope")
        .version("0.1.0")
        .author("MTRNord <info@nordgedanken.de>")
        .about("Converts OpenStreetMap Data to Euroscope Radar Screens")
        .subcommand(SubCommand::with_name("taxiways")
            .about("Converts taxiways to Euroscope Ground Networks")
            .arg(Arg::with_name("osm_data")
                .short("i")
                .long("osm_data")
                .value_name("osm.pbf FILE")
                .takes_value(true)
                .required(true)
                .help("The input data  used to convert from. (smaller data is faster). Needs to be .osm.pbf!"))
            .arg(Arg::with_name("airport")
                .short("a")
                .long("airport")
                .value_name("ICAO ID")
                .takes_value(true)
                .required(true)
                .help("The ICAO identifier of the Airport which should be converted")))
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("taxiways") {
        //generate_ese_ground_taxiway("./data/EDDH_cleaned.osm.pbf", "EDDH");
        generate_ese_ground_taxiway(matches.value_of("osm_data").unwrap(), matches.value_of("airport").unwrap());
    } else {
        println!("Please use the taxiways sub command!");
        exit(1);
    }
}

#[cfg(test)]
mod tests;
