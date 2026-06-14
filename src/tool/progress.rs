//! Live tool-output progress bridge.

use crate::session::SessionEvent;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use tokio::sync::mpsc;

static SINKS: OnceLock<Mutex<HashMap<String, Sink>>> = OnceLock::new();

#[derive(Clone)]
struct Sink {
    name: String,
    tx: mpsc::Sender<SessionEvent>,
}

pub struct Guard {
    id: String,
}

pub fn register(id: &str, name: &str, tx: &mpsc::Sender<SessionEvent>) -> Guard {
    sinks().lock().unwrap().insert(
        id.to_string(),
        Sink {
            name: name.to_string(),
            tx: tx.clone(),
        },
    );
    Guard { id: id.to_string() }
}

pub fn emit(id: &str, stream: &str, chunk: String) {
    let Some(sink) = sinks().lock().unwrap().get(id).cloned() else {
        return;
    };
    let _ = sink.tx.try_send(SessionEvent::ToolOutputChunk {
        tool_call_id: id.to_string(),
        name: sink.name,
        stream: stream.to_string(),
        chunk,
    });
}

fn sinks() -> &'static Mutex<HashMap<String, Sink>> {
    SINKS.get_or_init(|| Mutex::new(HashMap::new()))
}

impl Drop for Guard {
    fn drop(&mut self) {
        sinks().lock().unwrap().remove(&self.id);
    }
}
