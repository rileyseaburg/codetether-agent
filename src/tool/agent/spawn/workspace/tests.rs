//! Git-backed checks for child checkout isolation and explicit integration.
//!
//! Fixtures use only managed allocation beneath their temporary repository.

#[path = "ancestry_tests.rs"]
mod ancestry;
#[path = "concurrency_tests.rs"]
mod concurrency;
#[path = "failure_tests.rs"]
mod failure;
#[path = "handoff_tests.rs"]
mod handoff;
#[path = "integration_tests.rs"]
mod integration;
#[path = "isolation_tests.rs"]
mod isolation;
#[path = "test_support.rs"]
mod support;
