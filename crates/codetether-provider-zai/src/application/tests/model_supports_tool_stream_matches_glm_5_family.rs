//! Migrated application provider regression.
use super::*;
#[test]
fn model_supports_tool_stream_matches_glm_5_family() {
    assert!(ZaiProvider::model_supports_tool_stream("glm-5.3"));
    assert!(ZaiProvider::model_supports_tool_stream("glm-5.2"));
    assert!(ZaiProvider::model_supports_tool_stream("glm-5.1"));
    assert!(ZaiProvider::model_supports_tool_stream("glm-5-turbo"));
    assert!(!ZaiProvider::model_supports_tool_stream("glm-4.5"));
}
