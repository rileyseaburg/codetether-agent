use crate::provider::ModelInfo;
use anyhow::Result;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

use super::test_support_provider;

pub(super) struct ContextErrorUntilCompactProvider {
    pub(super) calls: AtomicUsize,
    pub(super) saw_compact_schema: AtomicBool,
}
impl ContextErrorUntilCompactProvider {
    pub(super) fn new() -> Self {
        Self { calls: AtomicUsize::new(0), saw_compact_schema: AtomicBool::new(false) }
    }
    pub(super) fn calls(&self) -> usize { self.calls.load(Ordering::SeqCst) }
    pub(super) fn saw_compact_schema(&self) -> bool { self.saw_compact_schema.load(Ordering::SeqCst) }
}
pub(super) fn provider_arc() -> Arc<ContextErrorUntilCompactProvider> {
    Arc::new(ContextErrorUntilCompactProvider::new())
}
