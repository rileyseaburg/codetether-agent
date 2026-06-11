#[path = "sandbox_seccomp_bpf.rs"]
mod bpf;
#[path = "sandbox_seccomp_fd.rs"]
mod fd;

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, RawFd};

#[derive(Debug)]
pub(crate) struct Program {
    file: File,
}

impl Program {
    pub(crate) fn fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

pub(crate) fn prepare(allow_network: bool) -> Result<Option<Program>> {
    let mut file = tempfile::tempfile().context("create seccomp profile file")?;
    file.write_all(&bpf::program(!allow_network))
        .context("write seccomp profile")?;
    file.seek(SeekFrom::Start(0))
        .context("rewind seccomp profile")?;
    fd::make_inheritable(file.as_raw_fd())?;
    Ok(Some(Program { file }))
}

#[cfg(test)]
#[path = "sandbox_seccomp_linux_tests.rs"]
mod tests;
