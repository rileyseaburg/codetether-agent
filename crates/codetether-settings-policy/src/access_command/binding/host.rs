//! Frontend session configuration and result presentation for access commands.
/// Implementation detail of [crate::declare_frontend_access_command].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_access_host {
    ($config:path, $message:path, $kind:path; $verifier:path, $release:path) => {
        mod host {
            use super::{App, Arc, ProviderRegistry, Session};
            use $config as Config;
            use $kind as MessageType;
            use $message as ChatMessage;
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub session: &'a mut Session,
                pub registry: Option<&'a Arc<ProviderRegistry>>,
            }
            impl $crate::access_command::Host for Native<'_> {
                fn verifier_command(&mut self, prompt: &str) -> bool {
                    $verifier(self.app, self.registry, prompt)
                }
                fn apply_session_config(&mut self, config: &Config) {
                    self.session
                        .apply_config(config, self.registry.map(Arc::as_ref));
                }
                fn release_all(&mut self) -> Result<usize, String> {
                    $release(self.app)
                }
                fn push_system(&mut self, text: String) {
                    self.app
                        .state
                        .messages
                        .push(ChatMessage::new(MessageType::System, text));
                }
                fn set_status(&mut self, status: String) {
                    self.app.state.status = status;
                }
                fn scroll_to_bottom(&mut self) {
                    self.app.state.scroll_to_bottom();
                }
                fn clear_input(&mut self) {
                    self.app.state.composer.clear_input();
                }
            }
        }
    };
}
