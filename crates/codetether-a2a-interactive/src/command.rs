//! Live-session A2A attachment command.
use crate::Presentation;
/// # Arguments
/// Receives the frontend bindings or active inbound slot and the operation's input.
/// # Returns
/// Applies the corresponding A2A transition while retaining the existing queue order.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_a2a_interactive::Presentation) {
/// codetether_a2a_interactive::command::handle(view, "");
/// # }
/// ```
pub fn handle(view: &mut impl Presentation, rest: &str) {
    let text = match rest.trim() {
        "accept" => {
            codetether_a2a_inbox::attach();
            "A2A: inbound peer turns now run in this session and appear in the chat. \
             Use `/a2a headless` to stop."
                .to_string()
        }
        "headless" => {
            codetether_a2a_inbox::detach();
            "A2A: inbound peer turns answered from a headless session.".to_string()
        }
        "" => format!(
            "A2A inbound mode: {}\n  /a2a accept    route peer turns into this session (default)\n  /a2a headless  answer peers headlessly",
            if codetether_a2a_inbox::is_attached() {
                "accept (live session)"
            } else {
                "headless"
            }
        ),
        other => format!("Unknown /a2a option `{other}`; use `accept` or `headless`."),
    };
    view.push_system(text);
    view.scroll_to_bottom();
}
