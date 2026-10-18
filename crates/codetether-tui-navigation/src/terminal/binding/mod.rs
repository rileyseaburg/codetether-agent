//! Frontend types and callbacks for terminal event routing.
mod host;
/// Declare terminal input dispatch over existing frontend handlers.
///
/// Key, editor-paste, ordinary paste and mouse callbacks retain their arguments;
/// the shared terminal policy controls routing priority and ignored events.
/// # Examples
/// See the frontend terminal module for the selected concrete types and handlers.
#[macro_export]
macro_rules! declare_frontend_terminal {
    ($app:path, $slot:path, $registry:path, $bridge:path, $runtime:path;
     $event:path, $key:path, $mouse:path;
     $key_handler:path, $editor_paste:path, $paste_handler:path, $mouse_handler:path $(,)?) => {
        use $app as App;
        use $slot as SessionSlot;
        use $registry as ProviderRegistry;
        use $bridge as TuiWorkerBridge;
        use $runtime as TuiSessionHandle;
        use $event as Event;
        use ::std::{path::Path, sync::Arc};
        $crate::__frontend_terminal_host!($key, $mouse; $key_handler, $editor_paste, $paste_handler, $mouse_handler);
        /// Dispatch one terminal event through the shared routing policy.
        /// # Errors
        /// Returns an error from the selected key-event handler.
        pub(super) async fn handle_terminal_event(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
    maybe: Option<Result<Event, std::io::Error>>,
) -> anyhow::Result<bool> {
    $crate::terminal::dispatch(
        &mut host::Native {
            app,
            cwd,
            slot,
            registry,
            worker_bridge,
            runtime,
        },
        maybe,
    )
    .await
}

    };
}
