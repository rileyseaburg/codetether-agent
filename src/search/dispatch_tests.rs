use super::enrich_args;
use crate::search::{Backend, BackendChoice};
use serde_json::json;

#[test]
fn memory_backend_is_forced_to_search_action() {
    let choice = BackendChoice {
        backend: Backend::Memory,
        args: json!({"action": "delete", "query": "keep this query"}),
    };
    let args = enrich_args(&choice, "fallback query");
    assert_eq!(args["action"], json!("search"));
    assert_eq!(args["query"], json!("keep this query"));
}
