//! Frontend shell, approval, mux and session intake operations.
/// Implementation detail of [crate::declare_frontend_command_intake].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_command_intake {
    ($shell:path, $approval:path, $mux:path, $session_free:path) => {
        mod intake {
            use super::{Native, session::Live};
            impl $crate::intake::Host for Native<'_> {
                async fn shell(&mut self, prompt: &str) -> bool {
                    $shell(self.app, self.cwd, prompt).await
                }
                fn approval(&mut self, prompt: &str) -> bool {
                    $approval(self.app, prompt)
                }
                async fn mux(&mut self, prompt: &str) -> bool {
                    $mux(self.app, self.cwd, prompt).await
                }
                async fn session(&mut self, prompt: &str) -> bool {
                    let Some(session) = self.slot.borrow_mut() else {
                        return false;
                    };
                    let mut live = Live {
                        app: self.app,
                        cwd: self.cwd,
                        session,
                        registry: self.registry,
                    };
                    $crate::intake::with_session(&mut live, prompt).await;
                    true
                }
                fn session_free(&mut self, prompt: &str) -> bool {
                    $session_free(self.app, self.cwd, self.registry, prompt)
                }
                fn clear_input(&mut self) {
                    self.app.state.composer.clear_input();
                }
                fn status(&mut self, message: String) {
                    self.app.state.status = message;
                }
            }
        }
    };
}
