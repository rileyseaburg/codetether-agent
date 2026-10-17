//! Runtime fixture that avoids network services and wall-clock dependencies.
use crate::runtime::Runtime;
use codetether_companion_protocol::SessionInput;

pub(super) fn runtime() -> Runtime {
    Runtime::new(SessionInput {
        model: "test/vision".into(),
        prompt: "Describe".into(),
        interval_seconds: 30,
    })
}
