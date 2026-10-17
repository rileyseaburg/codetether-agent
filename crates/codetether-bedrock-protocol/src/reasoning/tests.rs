//! Existing reasoning regression tests.

use super::ReasoningContentBlock;
use codetether_provider_core::ContentPart;

#[test]
fn signature_only_block_is_preserved() {
    let block: ReasoningContentBlock =
        serde_json::from_str(r#"{"reasoningText":{"signature":"CAIS=="}}"#).unwrap();
    let part = block.to_content_part().unwrap();
    assert!(matches!(
        part,
        ContentPart::Thinking { ref text, ref signature }
            if text.is_empty() && signature.as_deref() == Some("CAIS==")
    ));
}
