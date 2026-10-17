//! get task RPC operation.
use super::*;
pub(super) async fn get_task(
    service: &A2aServiceImpl,
    request: Request<proto::GetTaskRequest>,
) -> Result<Response<proto::Task>, Status> {
    let req = request.into_inner();
    let task_id = req.name.strip_prefix("tasks/").unwrap_or(&req.name);
    let task = service
        .store
        .get_task(task_id)
        .ok_or_else(|| Status::not_found(format!("task {task_id} not found")))?;
    Ok(Response::new(bridge::local_task_to_proto(&task)))
}
