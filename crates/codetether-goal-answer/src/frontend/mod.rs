//! Frontend journal, question input and held-answer delivery bindings.
mod actions;
mod delivery;
mod entry;
mod journal;
mod view;
/// Declare human-answer hosts using existing journal and frontend services.
///
/// The frontend retains session ownership and supplies durable user-journal
/// operations; shared answer policy continues to decide input and delivery order.
/// # Examples
/// See the frontend goal-answer module for concrete callbacks and types.
#[macro_export]
macro_rules! declare_frontend_answer_hosts {
    ($app:path, $slot:path, $registry:path, $runtime:path, $state:path, $action:path;
     $read:path, $held:path, $begin:path, $record:path;
     $restart:path, $ask:path $(,)?) => {
        use $app as App;
        use $slot as SessionSlot;
        use $registry as ProviderRegistry;
        use $runtime as TuiSessionHandle;
        use ::std::sync::Arc;
        $crate::__frontend_answer_journal!($state, $action; $read, $held, $begin, $record);
        $crate::__frontend_answer_delivery!($ask);
        mod input_host {
            use super::{App, SessionSlot, TuiSessionHandle};
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub slot: &'a SessionSlot,
                pub runtime: &'a TuiSessionHandle,
            }
            $crate::__frontend_answer_actions!($restart);
            $crate::__frontend_answer_view!();
        }
    };
}
