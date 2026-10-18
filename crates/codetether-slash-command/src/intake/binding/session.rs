//! Frontend retained-session access for parity and slash commands.
/// Implementation detail of [crate::declare_frontend_command_intake].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_command_session {
    ($session:path, $parity:path, $command:path) => {
        mod session {
            use super::{App, Arc, Path, ProviderRegistry};
            use $session as Session;
            pub(super) struct Live<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub session: &'a mut Session,
                pub registry: &'a Option<Arc<ProviderRegistry>>,
            }
            impl $crate::intake::SessionHost for Live<'_> {
                async fn parity(&mut self, prompt: &str) -> bool {
                    $parity(
                        self.app,
                        self.cwd,
                        self.session,
                        self.registry.as_ref(),
                        prompt,
                    )
                    .await
                }
                async fn command(&mut self, prompt: &str) {
                    $command(
                        self.app,
                        self.cwd,
                        self.session,
                        self.registry.as_ref(),
                        prompt,
                    )
                    .await;
                }
                fn clear_input(&mut self) {
                    self.app.state.clear_input();
                }
            }
        }
    };
}
