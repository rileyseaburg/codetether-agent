//! Frontend parked-turn admission and reply settlement.
/// Implementation detail of [crate::declare_frontend_inbox].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_inbox_live {
    ($inbound:path) => {
        pub(crate) mod live {
            use super::{App, IncomingTask, SessionSlot};
            use $inbound as Inbound;
            impl $crate::Inbox for App {
                fn processing(&self) -> bool {
                    self.state.processing
                }
                fn inbound(&mut self) -> &mut Option<Inbound> {
                    &mut self.state.live_inbound
                }
                fn enqueue_task(&mut self, task: IncomingTask) {
                    self.state.worker.enqueue_worker_task(task);
                }
            }
            pub(crate) fn admit(app: &mut App) {
                $crate::live::admit(app);
            }
            pub(crate) fn settle(app: &mut App, outcome: Result<String, String>) {
                $crate::live::settle(&mut app.state.live_inbound, outcome);
            }
            pub(crate) fn settle_from_session(app: &mut App, slot: &SessionSlot) {
                $crate::live::settle_from_session(
                    &mut app.state.live_inbound,
                    slot.borrow().map(|session| session.messages.as_slice()),
                );
            }
        }
    };
}
