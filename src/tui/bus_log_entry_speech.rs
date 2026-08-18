//! Retained display payload for one agent speech event.

/// Build (kind, summary, detail, color) for an agent speech event.
pub fn build(
    act: &str,
    from: &str,
    to: &str,
    conversation_id: &str,
    content: &str,
) -> (String, String, String, ratatui::style::Color) {
    let preview = crate::tui::bus_log_payload::summary(content);
    let retained = crate::tui::bus_log_payload::detail(content, "bus speech");
    (
        format!("SAY•{act}"),
        format!("{from} → {to}: {preview}"),
        format!(
            "Act: {act}\nFrom: {from}\nTo: {to}\nConversation: {conversation_id}\n\n{retained}"
        ),
        ratatui::style::Color::Magenta,
    )
}
