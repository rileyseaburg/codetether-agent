//! Frontend commands operations for queued-prompt delivery.
/// Implementation detail of [crate::declare_frontend_queued_prompt].
#[doc(hidden)]
#[macro_export]
macro_rules! __queued_prompt_commands {
    ($commands:path, $answers:path) => {
        mod commands {
            use super::Native;

            impl $crate::queued::Commands for Native<'_> {
                async fn commands(&mut self) {
                    $commands(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        self.runtime,
                    )
                    .await;
                }
                async fn answers(&mut self) {
                    $answers(self.app, self.slot, self.registry).await;
                }
            }
        }
    };
}
