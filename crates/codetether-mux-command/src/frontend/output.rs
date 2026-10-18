//! Frontend transcript and status presentation for mux command results.
/// Implementation detail of [crate::declare_frontend_mux_command].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_mux_output {
    ($message:path, $kind:path) => {
        mod output {
            use super::App;
            use ::anyhow::Result;
            use $kind as MessageType;
            use $message as ChatMessage;
            pub(super) fn show(app: &mut App, result: Result<String>) {
                let (kind, content, status) = match result {
                    Ok(content) => (MessageType::System, content, "Mux command completed"),
                    Err(error) => (
                        MessageType::Error,
                        format!("Mux error: {error}"),
                        "Mux command failed",
                    ),
                };
                app.state.messages.push(ChatMessage::new(kind, content));
                app.state.status = status.into();
                app.state.chat_viewport.scroll_to_bottom();
            }
        }
    };
}
