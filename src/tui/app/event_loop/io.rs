use tokio::sync::mpsc;

use crate::session::{Session, SessionEvent};

pub(crate) struct LoopIo<'a> {
    pub event_rx: &'a mut mpsc::Receiver<SessionEvent>,
    pub result_rx: &'a mut mpsc::Receiver<anyhow::Result<Session>>,
    pub shutdown_rx: &'a mut mpsc::Receiver<()>,
}

impl<'a> LoopIo<'a> {
    pub fn new(
        event_rx: &'a mut mpsc::Receiver<SessionEvent>,
        result_rx: &'a mut mpsc::Receiver<anyhow::Result<Session>>,
        shutdown_rx: &'a mut mpsc::Receiver<()>,
    ) -> Self {
        Self {
            event_rx,
            result_rx,
            shutdown_rx,
        }
    }
}
