//! Frontend services for prompt dispatch.
/// Implementation detail of [crate::declare_frontend_prompt_dispatch].
#[doc(hidden)]
#[macro_export]
macro_rules! __prompt_dispatch_services {
    ($image:path; $held:path, $started:path, $spawn:path, $missing:path) => {
        mod services {
            use super::super::{Arc, ProviderRegistry};
            use super::Native;
            use $crate::dispatch::{Host, State};
            use $image as ImageAttachment;
            impl Host for Native<'_> {
                type Provider = Arc<ProviderRegistry>;
                type Image = ImageAttachment;
                fn held(&self) -> bool {
                    $held(self.slot.view().id())
                }
                fn clear_input(&mut self) {
                    self.app.state.composer.clear_input();
                }
                async fn processing_started(&mut self) {
                    $started(self.app, self.worker_bridge).await;
                }
                fn begin_timing(&mut self) {
                    self.app
                        .state
                        .request_timing
                        .begin_with_activity(&mut self.app.state.watchdog.last_event_at);
                }
                fn state(&mut self) -> State<'_> {
                    State {
                        watchdog_restarts: &mut self.app.state.watchdog.restart_count,
                        inflight_prompt: &mut self.app.state.watchdog.inflight_prompt,
                        status: &mut self.app.state.status,
                    }
                }
                fn scroll(&mut self) {
                    self.app.state.chat_viewport.scroll_to_bottom();
                }
                async fn spawn(
                    &mut self,
                    provider: &Self::Provider,
                    prompt: &str,
                    images: Vec<Self::Image>,
                ) {
                    $spawn(
                        self.app,
                        self.cwd,
                        self.slot,
                        provider,
                        prompt,
                        images,
                        self.runtime,
                    )
                    .await;
                }
                async fn no_provider(&mut self) {
                    $missing(self.app, self.worker_bridge).await;
                }
            }
        }
    };
}
