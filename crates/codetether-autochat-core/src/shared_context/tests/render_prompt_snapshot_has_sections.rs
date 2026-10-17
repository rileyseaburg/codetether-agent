//! Existing shared-context regression.
use super::*;

#[test]
fn render_prompt_snapshot_has_sections() {
    let mut shared = SharedRelayContext::default();
    shared.facts.push("f1".to_string());
    let rendered = shared.render_prompt_snapshot();
    assert!(rendered.contains("Facts:"));
    assert!(rendered.contains("- f1"));
}
