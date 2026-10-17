//! Process-local sink storage shared by registration and emission.
use crate::Chunk;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

static SINKS: OnceLock<Mutex<HashMap<String, Sink>>> = OnceLock::new();

#[derive(Clone)]
pub(super) struct Sink {
    pub(super) name: String,
    pub(super) send: Arc<dyn Fn(Chunk) + Send + Sync>,
}

pub(super) fn sinks() -> &'static Mutex<HashMap<String, Sink>> {
    SINKS.get_or_init(|| Mutex::new(HashMap::new()))
}
