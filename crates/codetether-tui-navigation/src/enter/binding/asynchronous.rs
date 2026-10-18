//! Frontend asynchronous Enter effects.
/// Implementation detail of [crate::declare_frontend_enter].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_enter_asynchronous {
    ($chat:path) => {
        mod asynchronous {
            use super::Native;

            impl $crate::enter::Async for Native<'_> {
                async fn sessions(&mut self) {
                    super::super::mode::sessions(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        self.runtime,
                    )
                    .await;
                }
                async fn swarm(&mut self) {
                    super::super::enter_swarm::dispatch_swarm_enter(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        self.runtime,
                    )
                    .await;
                }
                async fn chat(&mut self) {
                    $chat(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        self.runtime,
                    )
                    .await;
                }
                async fn settings(&mut self) {
                    super::super::mode::settings(self.app, self.slot).await;
                }
            }
        }
    };
}
