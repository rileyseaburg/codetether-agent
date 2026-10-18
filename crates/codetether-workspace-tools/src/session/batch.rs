//! Weak batch binding after session tool filtering.
use crate::{Registry, composition::Factory};
use std::sync::Arc;
pub(super) fn with_batch<H: Factory>(mut registry: H::Registry) -> Arc<H::Registry> {
    let batch_tool = H::batch();
    registry.register(batch_tool.clone());
    let registry = Arc::new(registry);
    H::bind_batch(&batch_tool, Arc::downgrade(&registry));
    registry
}
