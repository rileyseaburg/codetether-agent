use tokio::sync::mpsc;

use crate::session::SessionEvent;
use crate::tui::app::session_runtime::SessionNotice;

pub(crate) struct LoopIo<'a> {
    pub event_rx: &'a mut mpsc::Receiver<SessionEvent>,
    pub notice_rx: &'a mut mpsc::Receiver<SessionNotice>,
    pub shutdown_rx: &'a mut mpsc::Receiver<()>,
}

impl<'a> LoopIo<'a> {
    pub fn new(
        event_rx: &'a mut mpsc::Receiver<SessionEvent>,
        notice_rx: &'a mut mpsc::Receiver<SessionNotice>,
        shutdown_rx: &'a mut mpsc::Receiver<()>,
    ) -> Self {
        Self {
            event_rx,
            notice_rx,
            shutdown_rx,
        }
    }
}
