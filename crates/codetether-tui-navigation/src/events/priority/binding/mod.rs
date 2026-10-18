//! Frontend resource selection for interrupt and modal priority dispatch.
mod host;
/// Declare priority-event handling with existing frontend modal callbacks.
/// # Examples
/// See the frontend event-priority module for concrete callback selection.
#[macro_export]
macro_rules! declare_frontend_event_priority {
    ($app:path, $slot:path, $runtime:path, $key:path;
     $interrupt:path, $interlude:path, $goal:path, $spawn:path, $fuzzy:path, $lsp:path, $editor:path $(,)?) => {
        use ::std::path::Path;
        use $app as App;
        use $key as KeyEvent;
        use $runtime as TuiSessionHandle;
        use $slot as SessionSlot;
        $crate::__frontend_event_priority_host!(
            $interrupt, $interlude, $goal, $spawn, $fuzzy, $lsp, $editor
        );
        pub(super) async fn handle(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            runtime: &TuiSessionHandle,
            key: KeyEvent,
        ) -> Option<bool> {
            $crate::events::priority::handle(
                &mut host::Native {
                    app,
                    cwd,
                    slot,
                    runtime,
                },
                key,
            )
            .await
        }
    };
}
