//! Cursor-gated frontend worker roster synchronization.
/// Declare the frontend's pre-draw worker synchronization entry point.
///
/// # Examples
/// ```text
/// codetether_tui_tick::declare_frontend_worker_sync!(pub(crate); App, Bridge, sync_agents);
/// ```
#[macro_export]
macro_rules! declare_frontend_worker_sync {
    ($visibility:vis; $app:ty, $bridge:ty, $sync:path $(,)?) => {
        $visibility fn before_draw(
    app: &mut $app,
    worker_bridge: &Option<$bridge>,
    synced_cursor: &mut Option<u64>,
) {
    let current = app.state.bus_cursor;
    $crate::worker_sync::before_draw(synced_cursor, current, || {
        $sync(app, worker_bridge);
    });
}

    };
}
