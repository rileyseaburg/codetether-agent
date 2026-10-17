//! Content-block inspection and ordering for repaired Bedrock turns.

use serde_json::Value;

mod ids;
mod results;

pub(super) fn tool_use_ids(message: &Value) -> Vec<String> {
    ids::tool_use_ids(message)
}

pub(super) fn put_required_results_first(message: &mut Value, ids: &[String]) {
    results::put_required_results_first(message, ids);
}

pub(super) fn drop_orphan_results(message: &mut Value) {
    results::drop_orphan_results(message);
}
