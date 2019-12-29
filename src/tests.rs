extern crate test;

use test::Bencher;

use super::*;

#[bench]
fn bench_eddh(b: &mut Bencher) {
    b.iter(|| generate_ese_ground_taxiway("./data/EDDH.osm.pbf", "EDDH1"));
}

#[bench]
fn bench_eddh_cleaned(b: &mut Bencher) {
    b.iter(|| generate_ese_ground_taxiway("./data/EDDH_cleaned.osm.pbf", "EDDH2"));
}

#[bench]
fn bench_hamburg(b: &mut Bencher) {
    b.iter(|| generate_ese_ground_taxiway("./data/hamburg-latest.osm.pbf", "EDDH3"));
}
