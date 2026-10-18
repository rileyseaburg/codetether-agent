use crate::strip_stats_header;

#[test]
fn strips_router_banner() {
    assert_eq!(strip_stats_header("[RLM: x]\n\nbody"), "body");
}
