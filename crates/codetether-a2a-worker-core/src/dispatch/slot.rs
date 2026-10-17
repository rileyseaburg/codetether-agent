//! Task-slot reservation helpers.

use std::{collections::HashSet, sync::Arc};

use tokio::sync::Mutex;

use super::TaskReservation;

/// Reserve capacity for a task without duplicating an active task ID.
///
/// # Arguments
/// * `processing` - Shared set of active task IDs.
/// * `task_id` - Task requesting a slot.
/// * `max_concurrent_tasks` - Maximum active set size.
/// # Returns
/// Whether the task was reserved, is already active, or exceeds capacity.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// use std::{collections::HashSet, sync::Arc};
/// use tokio::sync::Mutex;
/// use codetether_a2a_worker_core::dispatch::{reserve_task_slot, TaskReservation};
/// let active = Arc::new(Mutex::new(HashSet::new()));
/// assert_eq!(reserve_task_slot(&active, "task", 1).await, TaskReservation::Reserved);
/// # }
/// ```
pub async fn reserve_task_slot(
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
