//! Existing page policy regression.
use super::*;

#[test]
fn classify_routes_tool_results_to_evidence() {
    assert_eq!(
        classify(&tool_result("call-1", "output")),
        PageKind::Evidence
    );
}
