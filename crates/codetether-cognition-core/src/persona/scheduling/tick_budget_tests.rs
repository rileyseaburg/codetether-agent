//! Tests for per-tick budget window reset and work selection.

use chrono::Utc;

use super::tick_budget::select_work;
use super::tick_budget_tests_support::{persona, store};

#[test]
fn budget_counters_reset_after_window() {
    let mut personas = store(persona(61, 5_000, 3_000));
    let mut events = Vec::new();
    let work = select_work(&mut personas, Utc::now(), &mut events);

    let persona = &personas["p1"];
    assert_eq!(persona.tokens_this_window, 0);
    assert_eq!(persona.compute_ms_this_window, 0);
    assert_eq!(work.len(), 1, "persona should think after a window reset");
}

#[test]
fn exhausted_budget_pauses_persona_within_window() {
    let mut personas = store(persona(5, u32::MAX, 0));
    let mut events = Vec::new();
    let work = select_work(&mut personas, Utc::now(), &mut events);

    assert!(work.is_empty(), "over-budget persona should be skipped");
    assert!(personas["p1"].budget_paused);
    assert_eq!(events.len(), 1, "pausing should emit one event");
}
