//! Frontend terminal-handler context and event forwarding.
/// Implementation detail of [crate::declare_frontend_terminal].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_terminal_host {
    ($key:path, $mouse:path; $key_handler:path, $editor_paste:path, $paste_handler:path, $mouse_handler:path) => {
        mod host {
            use super::{
                App, Arc, Path, ProviderRegistry, SessionSlot, TuiSessionHandle, TuiWorkerBridge,
            };
            use $crate::terminal::Host;
            use $key as KeyEvent;
            use $mouse as MouseEvent;
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub slot: &'a mut SessionSlot,
                pub registry: &'a Option<Arc<ProviderRegistry>>,
                pub worker_bridge: &'a Option<TuiWorkerBridge>,
                pub runtime: &'a TuiSessionHandle,
            }
            impl Host for Native<'_> {
                type Error = anyhow::Error;
                async fn key(&mut self, key: KeyEvent) -> anyhow::Result<bool> {
                    $key_handler(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        self.runtime,
                        key,
                    )
                    .await
                }
                fn editor_paste(&mut self, text: &str) -> bool {
                    $editor_paste(self.app, text)
                }
                async fn paste(&mut self, text: &str) {
                    $paste_handler(self.app, text).await;
                }
                async fn mouse(&mut self, mouse: MouseEvent) {
                    $mouse_handler(self.app, self.cwd, mouse).await;
                }
            }
        }
    };
}
