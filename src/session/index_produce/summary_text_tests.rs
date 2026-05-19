//! Tests for [`crate::session::index_produce::summary_text`].

use super::summary_text::{clamp_tokens, strip_stats_header};

#[test]
fn strips_router_banner() {
    assert_eq!(strip_stats_header("[RLM: x]\n\nbody"), "body");
}

#[test]
fn clamps_long_text() {
    assert!(clamp_tokens(&"a".repeat(1000), 10).len() < 1000);
}
