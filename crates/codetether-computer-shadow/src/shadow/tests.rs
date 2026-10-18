//! Mocked-local planner and queue tests; no desktop/application claims.
mod coordinates;
mod keyboard;
mod mouse;
mod replay;
mod report;
mod status;
mod validation;
use super::super::input::ComputerUseInput;
use super::{
    plan,
    types::{Geometry, Plan, State},
};
use serde_json::{Value, json};

fn input(value: Value) -> ComputerUseInput {
    serde_json::from_value(value).unwrap()
}
fn planned(value: Value) -> Plan {
    plan::build(&input(value), Geometry::default(), State::default()).unwrap()
}

#[tokio::test]
#[cfg(not(windows))]
async fn linux_never_falls_back() {
    let result = super::dispatch(&input(json!({"action":"status","hwnd":1})))
        .await
        .unwrap();
    assert!(!result.success);
    assert!(result.output.contains("no physical fallback"));
}
