//! Poll construction from drained output and session activity.

use super::super::buffer::Buffer;
use super::super::{Poll, Running};

pub(super) fn build(command: &mut Running, output: Buffer) -> Poll {
    let poll_bytes = output.pushed();
    let (output, omitted_bytes) = output.finish();
    command.activity.record(poll_bytes);
    Poll {
        output,
        running: command.exit_code.is_none(),
        exit_code: command.exit_code,
        elapsed: command.activity.elapsed(),
        omitted_bytes,
        poll_bytes,
        silent_for: command.activity.silent(),
        session_bytes: command.activity.total_bytes(),
    }
}
