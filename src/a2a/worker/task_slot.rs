//! Task-slot reservation helpers.

use std::{collections::HashSet, sync::Arc};

use tokio::sync::Mutex;

use super::TaskReservation;

pub(super) async fn reserve_task_slot(
    processing: &Arc<Mutex<HashSet<String>>>,
    task_id: &str,
    max_concurrent_tasks: usize,
) -> TaskReservation {
    let mut proc = processing.lock().await;
    if proc.contains(task_id) {
        TaskReservation::AlreadyProcessing
    } else if proc.len() >= max_concurrent_tasks {
        TaskReservation::AtCapacity
    } else {
        proc.insert(task_id.to_string());
        TaskReservation::Reserved
    }
}
