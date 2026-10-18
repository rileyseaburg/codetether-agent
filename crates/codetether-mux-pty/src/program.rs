//! One persistent server-owned PTY program.

use std::fs::File;
use std::sync::{Arc, Mutex, atomic::AtomicBool};

use anyhow::Result;

use super::{TerminalSize, buffer::OutputBuffer};

pub(super) struct PtyProgram {
    pub(super) master: Mutex<File>,
    pub(super) output: Arc<Mutex<OutputBuffer>>,
    pub(super) running: Arc<AtomicBool>,
    pub(super) changed: tokio::sync::watch::Sender<u64>,
    pub(super) pid: u32,
}

impl PtyProgram {
    pub(super) fn start(
        command: &str,
        workspace: &std::path::Path,
        size: TerminalSize,
        mux_session: &str,
    ) -> Result<Self> {
        let (master, child) = super::spawn::open(command, workspace, size, mux_session)?;
        let reader = master.try_clone()?;
        let output = Arc::new(Mutex::new(OutputBuffer::new()));
        let running = Arc::new(AtomicBool::new(true));
        let (changed, _) = tokio::sync::watch::channel(0);
        let pid = child.id();
        super::reader::start(reader, output.clone(), running.clone(), changed.clone());
        super::monitor::start(child);
        Ok(Self {
            master: Mutex::new(master),
            output,
            running,
            changed,
            pid,
        })
    }
}
