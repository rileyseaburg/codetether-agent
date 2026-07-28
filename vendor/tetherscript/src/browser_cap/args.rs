//! Argument extraction for browser capability methods.

use crate::value::Value;

pub(crate) fn no_args(method: &str, args: &[Value]) -> Result<(), String> {
    args.is_empty()
        .then_some(())
        .ok_or_else(|| format!("browser.{} expects no arguments", method))
}

pub(crate) fn expect_str(method: &str, args: &[Value], index: usize) -> Result<String, String> {
    match args.get(index) {
        Some(Value::Str(s)) => Ok((**s).clone()),
        Some(v) => Err(format!(
            "browser.{} arg {} must be str, got {}",
            method,
            index + 1,
            v.type_name()
        )),
        None => Err(format!("browser.{} missing arg {}", method, index + 1)),
    }
}

pub(crate) fn expect_int(method: &str, args: &[Value], index: usize) -> Result<i64, String> {
    match args.get(index) {
        Some(Value::Int(n)) => Ok(*n),
        Some(Value::Float(f)) => Ok(*f as i64),
        Some(v) => Err(format!(
            "browser.{} arg {} must be int, got {}",
            method,
            index + 1,
            v.type_name()
        )),
        None => Err(format!("browser.{} missing arg {}", method, index + 1)),
    }
}

pub(crate) fn optional_int(args: &[Value], index: usize, default: i64) -> Result<i64, String> {
    match args.get(index) {
        None => Ok(default),
        Some(Value::Int(n)) => Ok(*n),
        Some(Value::Float(f)) => Ok(*f as i64),
        Some(v) => Err(format!("timeout/amount must be int, got {}", v.type_name())),
    }
}

pub(crate) fn expect_value(method: &str, args: &[Value], index: usize) -> Result<Value, String> {
    args.get(index)
        .cloned()
        .ok_or_else(|| format!("browser.{} missing arg {}", method, index + 1))
}
