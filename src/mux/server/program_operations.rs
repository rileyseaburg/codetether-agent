//! Response mapping for attached PTY operations.

use crate::mux::protocol::ServerResponse;
use crate::mux::pty::TerminalSize;

use super::context::ServerContext;

pub(super) fn attach(
    context: &ServerContext,
    id: u64,
    columns: u16,
    rows: u16,
) -> anyhow::Result<ServerResponse> {
    let offset = context
        .programs
        .attach(id, TerminalSize::new(columns, rows))?;
    Ok(ServerResponse::ProgramAttached {
        window_id: id,
        offset,
    })
}

pub(super) fn read(
    context: &ServerContext,
    id: u64,
    offset: u64,
) -> anyhow::Result<ServerResponse> {
    let chunk = context.programs.read(id, offset)?;
    Ok(ServerResponse::ProgramOutput {
        data: chunk.data,
        next_offset: chunk.next_offset,
        running: chunk.running,
    })
}

pub(super) fn resize(
    context: &ServerContext,
    id: u64,
    columns: u16,
    rows: u16,
) -> anyhow::Result<ServerResponse> {
    context
        .programs
        .resize(id, TerminalSize::new(columns, rows))?;
    Ok(ServerResponse::Acknowledged)
}
