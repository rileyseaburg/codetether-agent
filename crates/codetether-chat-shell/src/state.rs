//! In-flight background command ownership.
use crate::{ShellEvent, run::run};
use std::path::Path;
use tokio::sync::mpsc;
/// Owns the result receiver and busy flag for a chat shell command.
/// # Examples
/// ```
/// let state = codetether_chat_shell::State::default();
/// assert!(!state.is_running());
/// ```
#[derive(Default)]
pub struct State {
    receiver: Option<mpsc::UnboundedReceiver<ShellEvent>>,
    running: bool,
}
impl State {
    /// Whether a launched command still awaits delivery to the transcript.
    pub fn is_running(&self) -> bool {
        self.running
    }
    pub(crate) fn start(&mut self, cwd: &Path, command: String) {
        let (tx, rx) = mpsc::unbounded_channel();
        self.receiver = Some(rx);
        self.running = true;
        let cwd = cwd.to_path_buf();
        tokio::spawn(async move {
            let event = run(command, &cwd).await;
            let _ = tx.send(event);
        });
    }
    pub(crate) fn take_latest(&mut self) -> Option<ShellEvent> {
        let mut latest = None;
        if let Some(rx) = self.receiver.as_mut() {
            while let Ok(event) = rx.try_recv() {
                latest = Some(event);
            }
        }
        let event = latest?;
        self.receiver = None;
        self.running = false;
        Some(event)
    }
}
