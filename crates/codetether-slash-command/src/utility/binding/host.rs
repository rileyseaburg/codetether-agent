//! Frontend effects selected by utility-command routing.
/// Implementation detail of [crate::declare_frontend_utility].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_utility_host {
    ($access:path, $status:path, $diff:path, $copy:path, $dispatch:path) => {
        mod host {
            use super::{App, Arc, Path, ProviderRegistry, Session};
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub session: &'a mut Session,
                pub registry: Option<&'a Arc<ProviderRegistry>>,
            }
            impl $crate::utility::Host for Native<'_> {
                async fn access_mode(&mut self, prompt: &str) -> bool {
                    $access(self.app, self.cwd, self.session, self.registry, prompt).await
                }
                async fn status(&mut self, label: &str) {
                    $status(self.app, self.cwd, self.session, label).await;
                }
                fn diff(&mut self) {
                    $diff(self.app, self.cwd);
                }
                fn copy(&mut self, prompt: &str) {
                    $copy(self.app, prompt);
                }
                fn review(&mut self) {
                    $crate::utility::review::prepare(self.app);
                }
                async fn dispatch(&mut self, command: &str) {
                    $dispatch(self.app, self.cwd, self.session, self.registry, command).await;
                }
                fn clear_input(&mut self) {
                    self.app.state.clear_input();
                }
            }
        }
    };
}
