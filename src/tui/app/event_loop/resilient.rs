//! Resilient event-processing wrapper.
//!
//! Wraps `select_once` so that non-fatal errors (a bad terminal event, a
//! session event that triggers an internal error) are logged and swallowed
//! instead of tearing down the entire TUI. Only a genuine quit (`Ok(true)`)
//! breaks the loop.
//!
//! This is the difference between a TUI that survives a long-running session
//! and one that exits on the first hiccup.

pub(super) async fn run_resilient(
    args: &mut super::select_args::SelectArgs<'_>,
) -> bool {
    match super::select_loop::select_once(args).await {
        Ok(quit) => quit,
        Err(err) => {
            tracing::error!(%err, "event loop iteration failed — recovering");
            args.app.state.needs_redraw = true;
            false
        }
    }
}
