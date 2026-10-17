//! Payload parsing shared by the migrated spawn regression tests.
use crate::Params;

pub(super) fn params(value: serde_json::Value) -> Params {
    serde_json::from_value(value).expect("params parse")
}
