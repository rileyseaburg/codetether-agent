//! Proposed exec-policy amendment extraction from tool arguments.

use crate::approval::ExecPolicyAmendment;
use serde_json::Value;

pub(super) fn from_args(args: &Value) -> Option<ExecPolicyAmendment> {
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
    Some(ExecPolicyAmendment::new(tokens))
}
