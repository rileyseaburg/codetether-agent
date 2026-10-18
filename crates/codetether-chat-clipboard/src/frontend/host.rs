//! Frontend transcript, attachment queue and paste effects for clipboard actions.
/// Implementation detail of [crate::declare_frontend_chat_clipboard].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_chat_clipboard_host {
    ($app:path, $message:path, $image:path; $bottom:path, $paste:path) => {
        mod chat_clipboard_host {
            use $app as App;
            use $image as ImageAttachment;
            use $message as ChatMessage;
            pub(super) struct Native<'a>(pub &'a mut App);
            impl $crate::Host for Native<'_> {
                fn messages(&self) -> &[ChatMessage] {
                    &self.0.state.messages
                }
                fn pending_images(&mut self) -> &mut Vec<ImageAttachment> {
                    &mut self.0.state.pending_images
                }
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
                fn scroll_bottom(&mut self) {
                    self.0.state.chat_viewport.chat_scroll = $bottom;
                }
                fn paste_into_chat(&mut self, text: &str) {
                    $paste(self.0, text);
                }
            }
        }
    };
}
