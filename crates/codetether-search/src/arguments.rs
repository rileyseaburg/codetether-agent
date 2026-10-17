//! Normalize backend arguments and supply missing query fields.
use crate::types::{Backend, BackendChoice};
use serde_json::{Value, json};

/// Supply missing query fields and constrain memory operations to search.
///
/// # Arguments
/// * `choice` - Selected backend and its proposed arguments.
/// * `query` - Original query used when a backend field is missing.
/// # Returns
/// An owned JSON object with the backend's required defaults applied.
/// # Examples
/// ```
/// use codetether_search::{Backend, BackendChoice, arguments::enrich_args};
/// let choice = BackendChoice { backend: Backend::Memory, args: serde_json::Value::Null };
/// assert_eq!(enrich_args(&choice, "question")["action"], "search");
/// ```
pub fn enrich_args(choice: &BackendChoice, query: &str) -> Value {
    let mut args = choice.args.clone();
    if !args.is_object() {
        args = Value::Object(Default::default());
    }
    let obj = args.as_object_mut().expect("args normalized above");
    match choice.backend {
        Backend::Grep if obj.get("pattern").is_none() => {
            obj.insert("pattern".into(), json!(query));
        }
        Backend::Glob if obj.get("pattern").is_none() => {
            obj.insert("pattern".into(), json!(query));
        }
        Backend::Websearch if obj.get("query").is_none() => {
            obj.insert("query".into(), json!(query));
        }
        Backend::Memory => {
            obj.insert("action".into(), json!("search"));
            if obj.get("query").is_none() {
                obj.insert("query".into(), json!(query));
            }
        }
        _ => {}
    }
    args
}
#[cfg(test)]
#[path = "arguments_tests.rs"]
mod tests;
