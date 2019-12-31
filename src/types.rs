#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub(crate) struct LatLon {
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

pub(crate) struct Node {
    pub(crate) id: i64,
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

pub(crate) type Coords = Vec<LatLon>;
pub(crate) type Refs = Vec<i64>;
