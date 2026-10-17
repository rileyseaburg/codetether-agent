//! Loop emit progress.

use crate::router::types::{CrateAutoProcessContext, ProcessProgress};

pub(in crate::router::auto_loop) fn emit_progress(
    ctx: &CrateAutoProcessContext<'_>,
    iter: usize,
    max: usize,
) {
    if let Some(ref p) = ctx.on_progress {
        p(ProcessProgress {
            iteration: iter,
            max_iterations: max,
            status: "running".into(),
        });
    }
    if let Some(ref bus) = ctx.bus {
        let trace_id = ctx.trace_id.unwrap_or_else(uuid::Uuid::new_v4);
        bus.emit_progress(crate::RlmProgressEvent {
            trace_id,
            iteration: iter,
            max_iterations: max,
            status: "running".into(),
        });
    }
}
