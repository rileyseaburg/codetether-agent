//! cancel task RPC operation.
use super::*;
pub(super) async fn cancel_task(
    service: &A2aServiceImpl,
    request: Request<proto::CancelTaskRequest>,
) -> Result<Response<proto::Task>, Status> {
    let req = request.into_inner();
    let task_id = req.name.strip_prefix("tasks/").unwrap_or(&req.name);

    let mut task = service
        .store
        .tasks
        .get_mut(task_id)
        .ok_or_else(|| Status::not_found(format!("task {task_id} not found")))?;

    if task.status.state.is_terminal() {
        return Err(Status::failed_precondition(
            "task already in terminal state",
        ));
    }

    task.status = local::TaskStatus {
        state: TaskState::Cancelled,
        message: None,
        timestamp: Some(chrono::Utc::now().to_rfc3339()),
    };
    let snapshot = task.clone();
    drop(task);

    let _ = service
        .store
        .update_tx
        .send((task_id.to_string(), snapshot.status.clone()));

    Ok(Response::new(bridge::local_task_to_proto(&snapshot)))
}
