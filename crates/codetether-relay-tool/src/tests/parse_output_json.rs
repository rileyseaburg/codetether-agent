//! Relay regression coverage.
use super::*;

pub(super) fn parse_output_json(output: &str) -> Value {
    serde_json::from_str(output).expect("tool output should be valid json")
}
