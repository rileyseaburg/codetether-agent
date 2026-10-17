//! Aliases preserve execution and fallback suggestions preserve catalog order.
use super::fixture::Echo;
use crate::{AliasTool, Tool};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn alias_preserves_metadata_and_forwards_arguments() {
    let alias = AliasTool::new("legacy_echo", Arc::new(Echo));
    assert_eq!(alias.id(), "legacy_echo");
    assert_eq!(alias.name(), "Echo");
    assert_eq!(alias.description(), "Return the supplied input");
    assert_eq!(alias.parameters(), json!({"type":"object"}));
    let input = json!({"value":7});
    let result = alias.execute(input.clone()).await.unwrap();
    assert!(result.success);
    assert_eq!(result.metadata["input"], input);
}
