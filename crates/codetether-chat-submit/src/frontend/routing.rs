//! Frontend routing operations for chat submission.
/// Implementation detail of [crate::declare_frontend_chat_submit].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_chat_routing {
    ($capture:path, $managed:path, $mentions:path, $continuation:path, $slash:path) => {
        mod routing {
            use super::Native;
            impl $crate::Routing for Native<'_> {
                fn capture(&mut self) -> String {
                    $capture(self.app)
                }
                async fn managed(&mut self, prompt: &str) -> bool {
                    $managed(self.app, prompt).await
                }
                fn mentions(&self, prompt: &str) -> String {
                    $mentions(prompt, self.app.state.agent_tree.active.as_deref())
                }
                async fn continuation(&mut self, prompt: &str) -> bool {
                    $continuation(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        self.runtime,
                        prompt,
                    )
                    .await
                }
                async fn slash(&mut self, prompt: &str) -> bool {
                    $slash(self.app, self.cwd, self.slot, self.registry, prompt).await
                }
            }
        }
    };
}
