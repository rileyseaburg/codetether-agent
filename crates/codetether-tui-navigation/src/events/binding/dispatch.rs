//! Frontend dispatch operations for ordered key-event dispatch.
/// Implementation detail of [crate::declare_frontend_key_dispatch].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_key_dispatch {
    ($priority:path, $modified:path, $unmodified:path) => {
        mod dispatch {
            use super::super::KeyEvent;
            use super::Native;
            use ::std::time::Instant;
            use $crate::events::Dispatch;
            impl Dispatch for Native<'_> {
                async fn priority(&mut self, key: KeyEvent) -> Option<bool> {
                    $priority(self.app, self.cwd, self.slot, self.runtime, key).await
                }
                fn modified(&mut self, key: KeyEvent) -> Option<anyhow::Result<bool>> {
                    $modified(self.app, self.cwd, self.runtime, key)
                }
                async fn unmodified(&mut self, key: KeyEvent) -> anyhow::Result<bool> {
                    $unmodified(
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
                fn last_key(&mut self) -> &mut Option<Instant> {
                    &mut self.app.state.last_key_at
                }
            }
        }
    };
}
