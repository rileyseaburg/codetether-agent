use std::time::Duration;

use crossterm::event::EventStream;

use super::LoopTimers;
use crate::tui::constants::MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS;

pub(super) struct LoopSetup {
    pub reader: EventStream,
    pub shutdown_rx: tokio::sync::mpsc::Receiver<()>,
    pub timers: LoopTimers,
    pub worker_sync_cursor: Option<u64>,
}

pub(super) fn create() -> LoopSetup {
    let wd = Duration::from_secs(MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS);
    LoopSetup {
        reader: EventStream::new(),
        shutdown_rx: crate::tui::app::signal_shutdown::spawn_shutdown_listener(),
        timers: LoopTimers::new(wd),
        worker_sync_cursor: None,
    }
}
