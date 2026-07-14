//! One persistent server-owned PTY program.

use std::fs::File;
use std::sync::{Arc, Mutex, atomic::AtomicBool};

use anyhow::Result;

use super::{TerminalSize, buffer::OutputBuffer};

pub(super) struct PtyProgram {
    pub(super) master: Mutex<File>,
    pub(super) output: Arc<Mutex<OutputBuffer>>,
    pub(super) running: Arc<AtomicBool>,
    pub(super) pid: u32,
}

impl PtyProgram {
    pub(super) fn start(
        command: &str,
        workspace: &std::path::Path,
        size: TerminalSize,
    ) -> Result<Self> {
        let (master, child) = super::spawn::open(command, workspace, size)?;
        let reader = master.try_clone()?;
        let output = Arc::new(Mutex::new(OutputBuffer::new()));
        let running = Arc::new(AtomicBool::new(true));
        let pid = child.id();
        super::reader::start(reader, output.clone(), running.clone());
        super::monitor::start(child);
        Ok(Self {
            master: Mutex::new(master),
            output,
            running,
            pid,
        })
    }
}
