//! Helpers for parsing string lists from capability parameters.

use crate::value::Value;

/// Extract a `Vec<String>` from a tetherscript `Value`.
///
/// Accepts either a single `Str` (wrapped into a one-element list) or a
/// `List` of `Str` values. Returns an error with `context` on type mismatch.
pub(crate) fn string_list(val: &Value, context: &str) -> Result<Vec<String>, String> {
    match val {
        Value::Str(s) => Ok(vec![(**s).clone()]),
        Value::List(items) => {
            let borrow = items.borrow();
            let mut out = Vec::with_capacity(borrow.len());
            for item in borrow.iter() {
                match item {
                    Value::Str(s) => out.push((**s).clone()),
                    _ => return Err(format!("{}: all items must be strings", context)),
                }
            }
            Ok(out)
        }
        _ => Err(format!(
            "{}: expected string or list of strings, got {}",
            context,
            val.type_name()
        )),
    }
}
