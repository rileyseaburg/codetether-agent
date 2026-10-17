//! Default objective and key results for an autonomous task.

use codetether_okr::{KeyResult, Okr};
use uuid::Uuid;

/// Create a default OKR with standard key results for a task
pub(crate) fn create_default_okr(okr_id: Uuid, task: &str) -> Okr {
    let title = if task.len() > 60 {
        format!("Go: {}…", &task[..57])
    } else {
        format!("Go: {task}")
    };

    let mut okr = Okr::new(title, format!("Autonomous execution: {task}"));
    okr.id = okr_id;

    okr.add_key_result(KeyResult::new(okr_id, "All stories complete", 100.0, "%"));
    okr.add_key_result(KeyResult::new(okr_id, "Quality gates pass", 1.0, "count"));
    okr.add_key_result(KeyResult::new(okr_id, "No critical errors", 0.0, "count"));

    okr
}
