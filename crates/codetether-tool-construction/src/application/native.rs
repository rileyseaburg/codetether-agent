//! Search application-bound constructor groups lazily in their original order.
use super::Factories;
use crate::{Host, NativeTool};
use codetether_tool_core::Tool;
use std::sync::Arc;

pub(super) struct Adapter<'a, F>(pub &'a F);
impl<F: Factories> Host for Adapter<'_, F> {
    fn create(&self, kind: NativeTool) -> Arc<dyn Tool> {
        self.0
            .files(kind)
            .or_else(|| self.0.context(kind))
            .or_else(|| self.0.execution(kind))
            .or_else(|| self.0.integrations(kind))
            .expect("application-bound catalog constructor")
    }
}
