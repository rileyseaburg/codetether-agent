//! Intersection observer types.

#[derive(Clone, Debug, PartialEq)]
pub struct IntersectionEntry {
    pub target_node_id: u64,
    pub intersection_ratio: f64,
    pub is_intersecting: bool,
    pub bounds: (f64, f64, f64, f64),
}
