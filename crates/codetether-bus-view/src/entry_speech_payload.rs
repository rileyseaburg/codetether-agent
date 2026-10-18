//! Retained display payload for one agent speech event.

/// Build (kind, summary, detail, color) for an agent speech event.
/// # Arguments
/// Speech act, sender, recipient, conversation ID and text.
/// # Returns
/// The event kind, summary, detail and display color.
/// # Examples
/// ```
/// let (kind, _, _, _) = codetether_bus_view::entry_payload::speech("inform", "a", "b", "c", "hello");
/// assert_eq!(kind, "SAY•inform");
/// ```
pub fn build(
    act: &str,
    from: &str,
    to: &str,
    conversation_id: &str,
    content: &str,
) -> (String, String, String, ratatui::style::Color) {
    let preview = codetether_view_retention::bus::summary(content);
    let retained = codetether_view_retention::bus::detail(content, "bus speech");
    (
        format!("SAY•{act}"),
        format!("{from} → {to}: {preview}"),
        format!(
            "Act: {act}\nFrom: {from}\nTo: {to}\nConversation: {conversation_id}\n\n{retained}"
        ),
        ratatui::style::Color::Magenta,
    )
}
