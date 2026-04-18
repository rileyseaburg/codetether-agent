use chromiumoxide::{browser::Browser, page::Page};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::{
    sync::{Mutex, watch},
    task::JoinHandle,
};

pub(crate) struct SessionRuntime {
    pub alive: Arc<AtomicBool>,
    pub browser: Arc<Mutex<Browser>>,
    pub current_page: Arc<Mutex<Page>>,
    pub handler_task: JoinHandle<()>,
    pub mode: SessionMode,
    pub shutdown: watch::Sender<bool>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SessionMode {
    Launch,
    Connect,
}

impl SessionRuntime {
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
    }
}

impl SessionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Launch => "launch",
            Self::Connect => "connect",
        }
    }
}
