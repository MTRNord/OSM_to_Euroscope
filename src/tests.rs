extern crate test;

use test::Bencher;

use super::*;

#[bench]
fn bench_eddh(b: &mut Bencher) {
    b.iter(|| generate_ese_ground_taxiway( "EDDH1"));
}

#[bench]
fn bench_eddh_cleaned(b: &mut Bencher) {
    b.iter(|| generate_ese_ground_taxiway("EDDH2"));
}

#[bench]
fn bench_hamburg(b: &mut Bencher) {
    b.iter(|| generate_ese_ground_taxiway("EDDH3"));
}
