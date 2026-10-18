use super::Host;
use codetether_chat_model::{ChatMessage, MessageType};
pub(super) async fn resume<H: Host>(host: &mut H, prompt: String) {
    host.set_restarts(0);
    host.set_inflight(prompt);
    host.arm_retry(
        "Manual /continue — resubmitting stalled turn.".to_string(),
        0,
    );
    host.set_processing(false);
    host.clear_streaming();
    host.clear_timing();
    host.status("Continuing — resubmitting stalled turn…".to_string());
    host.messages_mut().push(ChatMessage::new(
        MessageType::System,
        "↻ /continue: cancelling stalled request and resubmitting.".to_string(),
    ));
    host.scroll();
    host.cancel().await;
}
