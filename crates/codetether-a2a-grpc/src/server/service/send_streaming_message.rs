//! send streaming message RPC operation.
use super::*;
pub(super) async fn send_streaming_message(
    service: &A2aServiceImpl,
    request: Request<proto::SendMessageRequest>,
) -> Result<Response<StreamResult<proto::StreamResponse>>, Status> {
    let req = request.into_inner();
    let msg = req
        .request
        .ok_or_else(|| Status::invalid_argument("missing message"))?;
    let local_msg = bridge::proto_message_to_local(&msg);

    let task_id = local_msg
        .task_id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let task = local::Task {
        id: task_id.clone(),
        context_id: local_msg.context_id.clone(),
        status: local::TaskStatus {
            state: TaskState::Submitted,
            message: Some(local_msg.clone()),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        },
        artifacts: vec![],
        history: vec![local_msg],
        metadata: Default::default(),
    };
    service.store.upsert_task(task.clone());

    // Return the initial task then listen for updates
    let proto_task = bridge::local_task_to_proto(&task);
    let rx = service.store.subscribe_updates();
    let tid = task_id.clone();

    let stream = send_streaming_message_stream::stream(proto_task, rx, tid);
    Ok(Response::new(stream))
}
