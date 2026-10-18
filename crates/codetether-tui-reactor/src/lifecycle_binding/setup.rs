//! Frontend input, signal and reporting resources for loop startup.
/// Declare loop resources and their existing startup sequence.
///
/// # Examples
/// ```text
/// codetether_tui_reactor::declare_frontend_loop_setup!(Reader, Reporter, TIMEOUT, shutdown_listener);
/// ```
#[macro_export]
macro_rules! declare_frontend_loop_setup {
    ($reader:path, $reporter:path, $timeout:path, $shutdown:path $(,)?) => {
        use ::std::time::Duration;
        use $crate::Timers as LoopTimers;
        use $reader as EventStream;
        use $reporter as Reporter;
        use $timeout as MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS;
        pub(super) struct LoopSetup {
            pub reader: EventStream,
            pub shutdown_rx: tokio::sync::mpsc::Receiver<()>,
            pub timers: LoopTimers,
            pub worker_sync_cursor: Option<u64>,
            pub mux_status: Reporter,
        }

        pub(super) fn create() -> LoopSetup {
            let wd = Duration::from_secs(MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS);
            LoopSetup {
                reader: EventStream::new(),
                shutdown_rx: $shutdown(),
                timers: LoopTimers::new(wd),
                worker_sync_cursor: None,
                mux_status: Reporter::default(),
            }
        }
    };
}
