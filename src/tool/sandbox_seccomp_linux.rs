#[path = "sandbox_seccomp_bpf.rs"]
mod bpf;
#[path = "sandbox_seccomp_fd.rs"]
mod fd;
#[path = "sandbox_seccomp_linux_apply.rs"]
mod install;

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, RawFd};

#[derive(Debug)]
pub(crate) struct Program {
    file: File,
    filters: std::sync::Arc<Vec<u64>>,
}

impl Program {
    pub(crate) fn fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

pub(crate) fn prepare(allow_network: bool) -> Result<Option<Program>> {
    let filters = bpf::program(!allow_network).context("unsupported seccomp architecture")?;
    let filters = std::sync::Arc::new(filters);
    let mut file = tempfile::tempfile().context("create seccomp profile file")?;
    file.write_all(bytemuck(&filters))
        .context("write seccomp profile")?;
    file.seek(SeekFrom::Start(0))
        .context("rewind seccomp profile")?;
    fd::make_inheritable(file.as_raw_fd())?;
    Ok(Some(Program { file, filters }))
}

pub(crate) fn apply(command: &mut tokio::process::Command, program: &Program) {
    install::tokio(command, program.filters.clone());
}

pub(crate) fn apply_std(command: &mut std::process::Command, program: &Program) {
    install::blocking(command, program.filters.clone());
}

fn bytemuck(filters: &[u64]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(filters.as_ptr().cast::<u8>(), std::mem::size_of_val(filters))
    }
}

#[cfg(test)]
#[path = "sandbox_seccomp_linux_tests.rs"]
mod tests;