use super::Ui;
use crate::WatchdogNotification;
use codetether_chat_model::{ChatMessage, MessageType};
/// Apply the UI state transition after detecting a stalled request.
/// Timeout is the existing user-visible no-event threshold in seconds.
pub fn apply<H: Ui>(host: &mut H, notification: WatchdogNotification, timeout: u64) {
    *host.state().restarts += 1;
    let count = *host.state().restarts;
    *host.state().notification = Some(notification);
    *host.state().processing = false;
    host.clear_streaming();
    host.clear_timing();
    *host.state().status = format!("Watchdog timeout — restarting (attempt {count})");
    host.state().messages.push(ChatMessage::new(
        MessageType::Error,
        format!("⚠ Watchdog: no events for {timeout}s. Auto-restarting (attempt {count})."),
    ));
    host.scroll();
}
