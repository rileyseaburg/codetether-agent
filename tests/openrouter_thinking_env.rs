//! Verifies the OpenRouter thinking level is seeded from the environment.
//!
//! Runs in its own test binary because `runtime_config` seeds once per
//! process, so the variable must be set before the first read.

use codetether_agent::provider::openrouter::runtime_config;

#[test]
fn thinking_level_is_seeded_from_the_environment() {
    // Safe here: single-threaded test binary, set before any provider call.
    unsafe { std::env::set_var("CODETETHER_OPENROUTER_THINKING_LEVEL", " XHIGH ") };

    assert_eq!(
        runtime_config::thinking_level().as_deref(),
        Some("xhigh"),
        "env value must be trimmed and lowercased on seed"
    );
}
