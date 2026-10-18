//! Frontend review operations for ordered key-event dispatch.
/// Implementation detail of [crate::declare_frontend_key_dispatch].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_key_review {
    ($review:path, $resume:path) => {
        mod review {
            use super::super::KeyEvent;
            use super::Native;
            use $crate::events::Review;
            impl Review for Native<'_> {
                type Error = anyhow::Error;
                async fn review(&mut self, key: KeyEvent) -> anyhow::Result<Option<bool>> {
                    $review(self.app, self.slot, key).await
                }
                async fn resume(&mut self) {
                    $resume(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        self.runtime,
                    )
                    .await;
                }
            }
        }
    };
}
