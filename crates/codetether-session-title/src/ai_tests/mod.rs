//! Automatic title policy regressions using counted, network-free providers.
mod bedrock;
mod inputs;
mod other;
mod provider;
mod session;

use super::generate_with;
use provider::MockProvider;
use session::Session;
use std::sync::atomic::{AtomicUsize, Ordering};

fn provider(name: &'static str, reply: Option<&'static str>) -> MockProvider {
    MockProvider {
        name,
        reply,
        calls: AtomicUsize::new(0),
    }
}
