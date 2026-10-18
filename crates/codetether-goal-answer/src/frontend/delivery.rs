//! Frontend delivery binding for human-answer workflows.
/// Implementation detail of [crate::declare_frontend_answer_hosts].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_answer_delivery {
    ($ask:path) => {
        mod delivery_host {
            use super::{App, Arc, ProviderRegistry, SessionSlot};
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub slot: &'a SessionSlot,
                pub registry: &'a Option<Arc<ProviderRegistry>>,
            }
            impl $crate::Delivery for Native<'_> {
                type Journal = super::review_host::Native;
                fn processing(&self) -> bool {
                    self.app.state.processing
                }
                fn available_session(&self) -> Option<String> {
                    self.slot.borrow().map(|session| session.id.clone())
                }
                async fn ask(&mut self, question: &str) -> bool {
                    let Some(session) = self.slot.borrow() else {
                        return false;
                    };
                    $ask(self.app, session, self.registry.as_ref(), question).await
                }
                fn status(&mut self, text: String) {
                    self.app.state.status = text;
                }
                fn answer_no(&mut self) {
                    self.app.state.answer_review_yes = false;
                }
                fn scroll(&mut self) {
                    self.app.state.chat_viewport.scroll_to_bottom();
                }
                fn redraw(&mut self) {
                    self.app.state.needs_redraw = true;
                }
            }
        }
    };
}
