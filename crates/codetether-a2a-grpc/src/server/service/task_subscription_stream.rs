//! Produce task status stream frames in subscription order.
use super::*;
pub(super) fn stream(
    proto_task: proto::Task,
    mut rx: broadcast::Receiver<(String, local::TaskStatus)>,
    tid: String,
) -> StreamResult<proto::StreamResponse> {
    let stream = async_stream::try_stream! {
        yield proto::StreamResponse {
            payload: Some(proto::stream_response::Payload::Task(proto_task)),
        };

        loop {
            match rx.recv().await {
                Ok((id, status)) if id == tid => {
                    let proto_status = bridge::local_task_status_to_proto(&status);
                    let is_terminal = status.state.is_terminal();
                    yield proto::StreamResponse {
                        payload: Some(proto::stream_response::Payload::StatusUpdate(
                            proto::TaskStatusUpdateEvent {
                                task_id: tid.clone(),
                                context_id: String::new(),
                                status: Some(proto_status),
                                r#final: is_terminal,
                                metadata: None,
                            },
                        )),
                    };
                    if is_terminal { break; }
                }
                Ok(_) => continue,
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    };
    Box::pin(stream) as StreamResult<proto::StreamResponse>
}
