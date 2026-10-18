//! Mock provider for the native checkpoint persistence integration test.

#[path = "restart_test_provider/checkpoint.rs"]
mod checkpoint;

pub(super) use checkpoint::CheckpointProvider;
