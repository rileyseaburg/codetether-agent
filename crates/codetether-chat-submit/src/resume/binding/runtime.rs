//! Frontend runtime effects for continuation and session resume.
/// Implementation detail of [crate::declare_frontend_continuation].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_continuation_runtime {
    ($dispatch:path) => {
        mod runtime {
            use super::Native;

            impl $crate::resume::Runtime for Native<'_> {
                async fn dispatch(&mut self, prompt: &str) {
                    $dispatch(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        prompt,
                        Vec::new(),
                        self.runtime,
                    )
                    .await;
                }
                async fn cancel(&self) {
                    self.runtime.cancel_current().await;
                }
            }
        }
    };
}
