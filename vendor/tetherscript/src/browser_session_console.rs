//! Console event normalization for browser sessions.

use crate::browser_session::ConsoleEvent;

pub(crate) fn event(message: String) -> ConsoleEvent {
    for level in ["error", "warn", "info", "debug"] {
        let prefix = format!("[{level}] ");
        if let Some(rest) = message.strip_prefix(&prefix) {
            return ConsoleEvent::new(level, rest);
        }
    }
    ConsoleEvent::new("log", message)
}
