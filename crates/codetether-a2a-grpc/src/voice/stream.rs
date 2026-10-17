//! Voice transport stream.
use super::*;
pub(super) fn stream(
    rx: tokio::sync::broadcast::Receiver<BusEnvelope>,
    room_name: String,
    topic_prefix: String,
) -> StreamResult<proto::VoiceEvent> {
    let stream = async_stream::try_stream! {
        let mut rx = rx;
        loop {
            match rx.recv().await {
                Ok(envelope) => {
                    if !envelope.topic.starts_with(&topic_prefix) {
                        continue;
                    }
                    match &envelope.message {
                        BusMessage::VoiceTranscript { room_name: _, text, role, is_final } => {
                            yield events_transcript::event(&room_name, text, role, *is_final, &envelope.timestamp);
                        }
                        BusMessage::VoiceAgentStateChanged { room_name: _, state } => {
                            yield events_state::event(&room_name, state);
                        }
                        BusMessage::VoiceSessionEnded { room_name: _, reason } => {
                            yield events_end::event(&room_name, reason);
                            break;
                        }
                        _ => {}
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            }
        }
    };
    Box::pin(stream) as StreamResult<proto::VoiceEvent>
}
