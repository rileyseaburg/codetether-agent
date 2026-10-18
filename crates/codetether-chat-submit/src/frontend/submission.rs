//! Frontend submission operations for chat submission.
/// Implementation detail of [crate::declare_frontend_chat_submit].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_chat_submission {
    ($steer:path, $finish:path) => {
        mod submission {
            use super::Native;
            impl $crate::Submission for Native<'_> {
                fn processing(&self) -> bool {
                    self.app.state.processing
                }
                fn steer(&mut self, prompt: &str) {
                    $steer(self.app, self.runtime, prompt);
                }
                async fn finish(&mut self, prompt: &str) {
                    $finish(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        self.runtime,
                        prompt,
                    )
                    .await;
                }
            }
        }
    };
}
