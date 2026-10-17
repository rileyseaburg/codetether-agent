//! Shared schema adapter for application integration coverage.
fn value() -> serde_json::Value {
    codetether_session_task::schema_value()
}
#[path = "separation_tests.rs"]
mod tests;
