use tokio::sync::mpsc;

use crate::session::SessionEvent;
use crate::tui::app::session_runtime::SessionNotice;

pub(super) struct SessionChannels {
    pub event_tx: mpsc::Sender<SessionEvent>,
    pub event_rx: mpsc::Receiver<SessionEvent>,
    pub notice_tx: mpsc::Sender<SessionNotice>,
    pub notice_rx: mpsc::Receiver<SessionNotice>,
}

pub(super) fn session() -> SessionChannels {
    let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(64);
    let (notice_tx, notice_rx) = mpsc::channel::<SessionNotice>(8);
    SessionChannels {
        event_tx,
        event_rx,
        notice_tx,
        notice_rx,
    }
}
