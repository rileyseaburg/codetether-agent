//! Compatibility adapter for native content search.
use anyhow::Result;
use serde_json::Value;
pub(super) fn run(args: &Value) -> Result<String> {
    codetether_search::native::grep(args)
}
