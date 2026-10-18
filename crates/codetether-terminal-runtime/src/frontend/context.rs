//! Frontend crash-context snapshot before a guarded draw.
/// Implementation detail of [crate::declare_frontend_safe_draw].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_draw_context {
    ($record:path) => {
        mod safe_draw_context {
            use super::{App, SessionView};
            pub(super) fn record(app: &App, session: &SessionView) {
                $record(
                    &session.id,
                    session.message_count,
                    session.model.as_deref(),
                    session.directory.as_deref(),
                    &app.state.status,
                );
            }
        }
    };
}
