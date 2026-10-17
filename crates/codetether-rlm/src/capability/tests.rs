//! Capability registry behavior.

use super::{OutputCapability, output_capability};

#[test]
fn classifies_known_tools() {
    assert_eq!(
        output_capability("webfetch"),
        OutputCapability::BulkSummarizable
    );
    assert_eq!(
        output_capability("session_context"),
        OutputCapability::BulkSummarizable
    );
    assert_eq!(output_capability("read"), OutputCapability::ExactContent);
    assert_eq!(output_capability("bash"), OutputCapability::ExactContent);
    assert_eq!(
        output_capability("session_recall"),
        OutputCapability::ExactContent
    );
    assert_eq!(
        output_capability("brand_new_tool"),
        OutputCapability::Unknown
    );
}
