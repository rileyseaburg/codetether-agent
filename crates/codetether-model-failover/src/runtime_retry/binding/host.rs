//! Frontend model, timing and session-view access for retries.
/// Implementation detail of [crate::declare_frontend_retry].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_retry_host {
    ($app:path, $slot:path, $registry:path, $runtime:path) => {
        mod host {
            use ::std::sync::Arc;
            use $app as App;
            use $crate::runtime_retry::{Host, State};
            use $registry as ProviderRegistry;
            use $runtime as TuiSessionHandle;
            use $slot as SessionSlot;
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub slot: &'a mut SessionSlot,
                pub runtime: &'a TuiSessionHandle,
            }
            impl Host for Native<'_> {
                type Provider = Arc<ProviderRegistry>;
                fn state(&mut self) -> State<'_> {
                    State {
                        retry: &mut self.app.state.retry_state,
                        processing: &mut self.app.state.processing,
                        inflight: &mut self.app.state.main_inflight_prompt,
                        last_event: &mut self.app.state.main_last_event_at,
                        status: &mut self.app.state.status,
                    }
                }
                fn current_model(&self) -> Option<Option<&str>> {
                    self.slot.borrow().map(|s| s.metadata.model.as_deref())
                }
                async fn set_model(&mut self, model: String) {
                    if let Some(session) = self.slot.borrow_mut() {
                        session.metadata.model = Some(model);
                        let _ = session.save().await;
                    }
                }
                fn refresh_view(&mut self) {
                    self.slot.refresh_view();
                }
                fn begin_timing(&mut self) {
                    self.app.state.begin_request_timing();
                }
                async fn submit(&mut self, prompt: &str, provider: &Self::Provider) {
                    super::submission::submit(self.slot, prompt, self.runtime, provider).await;
                }
            }
        }
    };
}
