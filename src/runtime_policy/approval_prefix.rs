//! Proposed exec-policy amendment extraction from tool arguments.

use crate::approval::ExecPolicyAmendment;
use serde_json::Value;

pub(super) fn from_args(tool_name: &str, args: &Value) -> Option<ExecPolicyAmendment> {
    let tokens = args
        .get("prefix_rule")?
        .as_array()?
        .iter()
        .filter_map(Value::as_str)
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    if tokens.is_empty() || tokens.iter().any(|t| t.contains(['\n', '\r'])) {
        return None;
    }
    let command = super::command::value(tool_name, args)?.trim_start();
    let prefix = tokens.join(" ");
    if super::command_unsafe::rejected(&prefix)
        || command != prefix && !command.starts_with(&format!("{prefix} "))
    {
        return None;
    }
    Some(ExecPolicyAmendment::new(tokens))
}

#[cfg(test)]
#[path = "approval_prefix_tests.rs"]
mod tests;
