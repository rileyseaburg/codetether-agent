//! Frontend interrupt and modal handlers in shared priority order.
/// Implementation detail of [crate::declare_frontend_event_priority].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_event_priority_host {
    ($interrupt:path, $interlude:path, $goal:path, $spawn:path, $fuzzy:path, $lsp:path, $editor:path) => {
        mod host {
            use super::{App, KeyEvent, Path, SessionSlot, TuiSessionHandle};
            use $crate::events::priority::Host;
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub slot: &'a mut SessionSlot,
                pub runtime: &'a TuiSessionHandle,
            }
            impl Host for Native<'_> {
                fn interrupt(&mut self, key: KeyEvent) -> Option<bool> {
                    $interrupt(self.app, self.runtime, key)
                }
                fn interlude(&mut self, key: KeyEvent) -> bool {
                    $interlude(self.app, key)
                }
                fn goal(&mut self, key: KeyEvent) -> bool {
                    $goal(self.app, key)
                }
                fn spawn_active(&self) -> bool {
                    self.app.state.spawn_form.is_some()
                }
                async fn spawn(&mut self, key: KeyEvent) -> bool {
                    $spawn(self.app, self.cwd, self.slot, key).await
                }
                fn fuzzy(&mut self, key: KeyEvent) -> bool {
                    $fuzzy(self.app, self.cwd, key)
                }
                async fn editor_lsp(&mut self, key: KeyEvent) -> bool {
                    $lsp(self.app, self.cwd, key).await
                }
                fn editor(&mut self, key: KeyEvent) -> bool {
                    $editor(self.app, self.cwd, key)
                }
            }
        }
    };
}
