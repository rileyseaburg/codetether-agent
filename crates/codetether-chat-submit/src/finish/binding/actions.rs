//! Frontend actions for prompt finish.
/// Implementation detail of [crate::declare_frontend_prompt_finish].
#[doc(hidden)]
#[macro_export]
macro_rules! __prompt_finish_actions {
    ($adopt:path, $publish:path, $dispatch:path) => {
        mod actions {
            use super::Native;
            impl $crate::finish::Actions for Native<'_> {
                async fn adopt(&mut self, agent_prompt: &str) {
                    $adopt(self.app, self.slot, agent_prompt).await;
                }
                fn publish(&mut self, prompt: &str, images: &[Self::Image]) {
                    $publish(self.app, prompt, images);
                }
                async fn dispatch(&mut self, agent_prompt: &str, images: Vec<Self::Image>) {
                    $dispatch(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        agent_prompt,
                        images,
                        self.runtime,
                    )
                    .await;
                }
            }
        }
    };
}
