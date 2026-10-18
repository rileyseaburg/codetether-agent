use super::Host;
use crate::{
    classify::{EventClass, classify},
    cursor::Cursor,
    frame::ParsedFrame,
};
/// Handle one parsed frame against the runtime, advancing `cursor` for
/// sequenced events only after their side effects are dispatched.
///
/// # Arguments
/// * `frame` — Parsed server event, including optional sequence ID.
/// * `host` — Native task dispatch and cold-reconciliation hooks.
/// * `cursor` — Durable resume position updated after sequenced dispatch.
///
/// # Returns
/// Completes after dispatch or resynchronization; cursor/poll errors are logged.
///
/// # Examples
/// ```ignore
/// handle_frame(&frame, &worker_hooks, &mut cursor).await;
/// ```
pub async fn handle_frame<H: Host>(frame: &ParsedFrame, host: &H, cursor: &mut Cursor) {
    if frame.event == "resync-required" {
        return super::resync::handle_resync(frame, host, cursor).await;
    }
    if frame.data == "[DONE]" || frame.data.is_empty() {
        return;
    }
    let Ok(task) = serde_json::from_str::<serde_json::Value>(&frame.data) else {
        return;
    };
    match classify(&frame.event) {
        EventClass::Control => {}
        EventClass::Advisory => host.dispatch(&task).await,
        EventClass::Sequenced => {
            host.dispatch(&task).await;
            if let Some(id) = &frame.id
                && let Err(error) = cursor.commit(id)
            {
                tracing::warn!(error = %error, "Failed to commit stream cursor");
            }
        }
    }
}
