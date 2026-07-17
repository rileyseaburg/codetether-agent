//! Interactive pipe fallback for non-Unix platforms.

use super::Attached;
use std::io;
use std::process::Stdio;

pub(super) fn attach(command: &mut tokio::process::Command) -> io::Result<Attached> {
    command.stdin(Stdio::piped());
    Ok(Attached::Pipes)
}
