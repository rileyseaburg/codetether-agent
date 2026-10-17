//! task subscription RPC operation.
use super::*;
pub(super) async fn task_subscription(
    service: &A2aServiceImpl,
    request: Request<proto::TaskSubscriptionRequest>,
) -> Result<Response<StreamResult<proto::StreamResponse>>, Status> {
    let req = request.into_inner();
    let task_id = req
        .name
        .strip_prefix("tasks/")
        .unwrap_or(&req.name)
        .to_string();

    let task = service
        .store
        .get_task(&task_id)
        .ok_or_else(|| Status::not_found(format!("task {task_id} not found")))?;

    let proto_task = bridge::local_task_to_proto(&task);
    let rx = service.store.subscribe_updates();
    let tid = task_id.clone();

    // If already terminal, return just the task and close
    if task.status.state.is_terminal() {
        let stream = async_stream::try_stream! {
            yield proto::StreamResponse {
                payload: Some(proto::stream_response::Payload::Task(proto_task)),
            };
        };
        return Ok(Response::new(
            Box::pin(stream) as StreamResult<proto::StreamResponse>
        ));
    }

    let stream = task_subscription_stream::stream(proto_task, rx, tid);
    Ok(Response::new(stream))
}
