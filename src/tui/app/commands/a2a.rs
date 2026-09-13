//! `/a2a` — control whether LAN peers may address this live session.
//!
//! * `/a2a accept`   route inbound peer turns into this session (opt in)
//! * `/a2a headless` answer peers from a headless session (default)
//! * `/a2a`          show the current mode

use crate::a2a::live_inbox;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) fn handle(app: &mut App, rest: &str) {
    let text = match rest.trim() {
        "accept" => {
            live_inbox::attach();
            "A2A: inbound peer turns now run in this session and appear in the chat. \
             Use `/a2a headless` to stop."
                .to_string()
        }
        "headless" => {
            live_inbox::detach();
            "A2A: inbound peer turns answered from a headless session.".to_string()
        }
        "" => format!(
            "A2A inbound mode: {}\n  /a2a accept    route peer turns into this session\n  /a2a headless  answer peers headlessly (default)",
            if live_inbox::is_attached() {
                "accept (live session)"
            } else {
                "headless"
            }
        ),
        other => format!("Unknown /a2a option `{other}`; use `accept` or `headless`."),
    };
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, text));
    app.state.scroll_to_bottom();
}
