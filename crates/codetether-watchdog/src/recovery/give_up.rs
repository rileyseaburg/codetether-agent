use super::Ui;
use crate::WATCHDOG_MAX_RESTARTS;
use codetether_chat_model::{ChatMessage, MessageType};
/// Stop automatic retry and report permanent request failure.
pub fn give_up<H: Ui>(host: &mut H) {
    let max = WATCHDOG_MAX_RESTARTS;
    *host.state().notification = None;
    *host.state().processing = false;
    host.clear_streaming();
    host.clear_timing();
    *host.state().status = format!("Watchdog gave up after {max} retries");
    host.state().messages.push(ChatMessage::new(MessageType::Error,format!("⚠ Watchdog: request stalled {max} times. The provider or tool may be permanently unavailable.

Press Enter to retry manually or switch providers with Ctrl+M.")));
    host.scroll();
}
