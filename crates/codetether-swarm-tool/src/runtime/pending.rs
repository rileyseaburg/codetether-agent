//! Guarded worker handles retained in task order.
use crate::{abort_on_drop::AbortOnDrop, model_selection::ModelSelection, task_result::TaskResult};
/// Position, failure identity, model diagnostics, and guarded worker handle.
pub(super) type Pending = (
    usize,
    String,
    String,
    ModelSelection,
    AbortOnDrop<anyhow::Result<TaskResult>>,
);
