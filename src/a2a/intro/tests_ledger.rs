//! Tests for the persistent intro ledger.

use crate::a2a::intro::ledger;
use crate::a2a::intro::tests_fixtures::with_temp_data_dir;

#[test]
fn ledger_normalizes_trailing_slash() {
    assert!(!ledger::contains("http://198.51.100.7:1/"));
}

#[test]
fn ledger_load_handles_missing_file() {
    with_temp_data_dir(|| {
        let set = ledger::load().expect("missing ledger is Ok(empty)");
        assert!(set.is_empty());
    });
}

#[test]
fn ledger_contains_is_idempotent_under_repeated_calls() {
    with_temp_data_dir(|| {
        let endpoint = "http://198.51.100.99:7";
        assert!(!ledger::contains(endpoint));
    });
}

#[test]
fn ledger_record_then_contains_round_trip() {
    with_temp_data_dir(|| {
        let endpoint = "http://203.0.113.42:41219";
        ledger::record(endpoint);
        assert!(ledger::contains(endpoint));
    });
}
