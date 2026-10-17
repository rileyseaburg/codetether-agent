//! Normalize stdout, return values and success status.
use serde_json::Value;
use tetherscript::value::{ResultValue, Value as TetherScriptValue};

/// Captured execution text, success state and the JSON-compatible return value.
///
/// # Examples
/// ```rust
/// let outcome=codetether_tetherscript_runtime::TetherScriptOutcome{
///     output:"done".into(),success:true,value:serde_json::json!(null),
/// };
/// assert!(outcome.success);
/// ```
#[derive(Debug)]
pub struct TetherScriptOutcome {
    pub output: String,
    pub success: bool,
    pub value: Value,
}

pub fn output(mut stdout: String, value: &TetherScriptValue) -> String {
    if !is_empty(value) {
        if !stdout.is_empty() && !stdout.ends_with('\n') {
            stdout.push('\n');
        }
        stdout.push_str(&value.to_string());
    }
    stdout
}

pub fn is_success(value: &TetherScriptValue) -> bool {
    !matches!(value, TetherScriptValue::Result(r) if matches!(r.as_ref(), ResultValue::Err(_)))
}

fn is_empty(value: &TetherScriptValue) -> bool {
    matches!(value, TetherScriptValue::Nil)
        || matches!(value, TetherScriptValue::Result(r) if matches!(r.as_ref(), ResultValue::Ok(TetherScriptValue::Nil)))
}
