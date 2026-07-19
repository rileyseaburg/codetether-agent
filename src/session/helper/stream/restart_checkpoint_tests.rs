//! Completed output-item recovery integration tests.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use super::restart_test_provider::CheckpointProvider;
use crate::provider::{CompletionRequest, Provider, StreamChunk};

#[path = "restart_checkpoint_tests/exhaustion.rs"]
mod exhaustion;
#[path = "restart_checkpoint_tests/persistence.rs"]
mod persistence;
#[path = "restart_checkpoint_tests/text.rs"]
mod text;
#[path = "restart_checkpoint_tests/tool.rs"]
mod tool;

fn request() -> CompletionRequest {
    CompletionRequest {
        messages: Vec::new(),
        tools: Vec::new(),
        model: "mock".into(),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: Vec::new(),
    }
}

fn provider(streams: Vec<Vec<StreamChunk>>) -> (Arc<CheckpointProvider>, Arc<dyn Provider>) {
    let concrete = Arc::new(CheckpointProvider {
        streams: Mutex::new(VecDeque::from(streams)),
        requests: Mutex::new(Vec::new()),
    });
    (concrete.clone(), concrete)
}
