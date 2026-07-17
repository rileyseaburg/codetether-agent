use super::{collect_peers, peer_candidates};

#[test]
fn peer_collection_normalizes_deduplicates_and_skips_self() {
    let peers = [
        "localhost:5000",
        "http://localhost:5000/",
        "http://localhost:5001",
    ]
    .map(ToString::to_string);
    assert_eq!(collect_peers(&peers, "http://localhost:5001").len(), 1);
}

#[test]
fn candidates_include_compatible_a2a_path() {
    assert_eq!(
        peer_candidates("http://localhost:4096"),
        ["http://localhost:4096", "http://localhost:4096/a2a"]
    );
    assert_eq!(
        peer_candidates("http://localhost:4096/a2a"),
        ["http://localhost:4096/a2a"]
    );
}
