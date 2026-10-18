//! Frontend resource and callback selection for reactor dispatch.
mod after;
mod events;
mod notices;
mod ticks;

/// Declare a frontend selection cycle over existing event handlers.
///
/// Supplies the dispatch host, its five operation groups, channel selection and
/// the `select_once` entry point. Callback groups retain their existing argument
/// order and async behavior; the shared dispatcher owns execution order.
///
/// # Examples
/// See the frontend's selection module for concrete resource and callback paths.
#[macro_export]
macro_rules! declare_frontend_selection {
    ($args:path, $snapshot:ty, $terminal_type:ty, $event:ty, $notice_type:ty;
     $capture:path;
     $terminal:path, $session:path, $coalesce:path;
     $drain:path, $queued:path, $inbox:path;
     $watchdog:path, $audit:path, $tick:path, $retry:path;
     $notice:path, $reconnect:path, $smart_retry:path, $watchdog_retry:path $(,)?) => {
        use $args as SelectArgs;
        mod host {
            use super::SelectArgs;
            pub(super) struct Native<'a, 'b>(pub &'a mut SelectArgs<'b>);
            impl $crate::dispatch::State for Native<'_, '_> {
                type Snapshot = $snapshot;
                fn snapshot(&self) -> Self::Snapshot {
                    $capture(self.0.app)
                }
                fn needs_redraw(&mut self) -> &mut bool {
                    &mut self.0.app.state.needs_redraw
                }
            }
            $crate::__reactor_selection_events!(
                $terminal_type, $event; $terminal, $session, $coalesce
            );
            $crate::__reactor_selection_after!($drain, $queued, $inbox);
            $crate::__reactor_selection_ticks!($watchdog, $audit, $tick, $retry);
            $crate::__reactor_selection_notices!(
                $notice_type; $notice, $reconnect, $smart_retry, $watchdog_retry
            );
        }
        mod wait {
            use super::SelectArgs;
            type Ready = $crate::Ready<$terminal_type, $event, $notice_type>;
            pub(super) async fn next(args: &mut SelectArgs<'_>) -> Ready {
                $crate::select(&mut $crate::Sources {
                    reader: &mut *args.reader,
                    shutdown: &mut *args.io.shutdown_rx,
                    events: &mut *args.io.event_rx,
                    notices: &mut *args.io.notice_rx,
                    timers: &mut *args.timers,
                })
                .await
            }
        }
        pub(super) async fn select_once(args: &mut SelectArgs<'_>) -> ::anyhow::Result<bool> {
            let before = $capture(args.app);
            let event = wait::next(args).await;
            $crate::dispatch::dispatch(&mut host::Native(args), event, before).await
        }
    };
}
