//! Frontend resources and callbacks for the shared loop driver.
mod cycle;
mod finish;
mod prepare;

/// Declare frontend loop resources and their driver implementations.
///
/// The frontend supplies concrete resource types and existing callbacks. The
/// declaration preserves channel ownership, per-iteration borrows and shutdown
/// resource consumption without introducing dependencies on frontend types.
///
/// # Examples
/// ```text
/// codetether_tui_reactor::declare_frontend_driver!(
///     TerminalType, App, Registry, Slot, Recorder, Bridge, Event, Runtime,
///     Notice, Setup; LoopIo, SelectArgs; before_draw, draw, select, finish,
/// );
/// ```
#[macro_export]
macro_rules! declare_frontend_driver {
    ($terminal:ty, $app:ty, $registry:ty, $slot:ty, $recorder:ty,
     $bridge:ty, $event:ty, $runtime:ty, $notice:ty, $setup:ty;
     $io:path, $args:path; $before:path, $draw:path, $select:path, $finish:path $(,)?) => {
        pub(super) struct Native<'a> {
            pub terminal: &'a mut $terminal,
            pub app: &'a mut $app,
            pub cwd: &'a ::std::path::Path,
            pub registry: Option<::std::sync::Arc<$registry>>,
            pub slot: &'a mut $slot,
            pub bus_recorder: ::std::sync::Arc<$recorder>,
            pub worker_bridge: Option<$bridge>,
            pub event_rx: ::tokio::sync::mpsc::Receiver<$event>,
            pub runtime: $runtime,
            pub notice_rx: ::tokio::sync::mpsc::Receiver<$notice>,
            pub setup: $setup,
            pub last_draw: Option<::std::time::Instant>,
        }
        $crate::__reactor_driver_prepare!($before, $draw);
        $crate::__reactor_driver_cycle!($io, $args, $select);
        $crate::__reactor_driver_finish!($finish);
    };
}
