//! Mapping PTY protocol requests onto the server-owned registry.

use crate::mux::protocol::{ClientRequest, ServerResponse};
use crate::mux::pty::TerminalSize;

use super::context::ServerContext;

pub(super) async fn execute(
    context: &ServerContext,
    request: ClientRequest,
) -> anyhow::Result<ServerResponse> {
    let response = match request {
        ClientRequest::StartProgram {
            window_id,
            command,
            columns,
            rows,
        } => {
            return super::program_start::start(
                context,
                window_id,
                &command,
                TerminalSize::new(columns, rows),
            )
            .await;
        }
        ClientRequest::AttachProgram {
            window_id,
            columns,
            rows,
        } => super::program_operations::attach(context, window_id, columns, rows)?,
        ClientRequest::ProgramInput { window_id, data } => {
            context.programs.input(window_id, &data)?;
            ServerResponse::Acknowledged
        }
        ClientRequest::ReadProgram { window_id, offset } => {
            super::program_operations::read(context, window_id, offset)?
        }
        ClientRequest::ResizeProgram {
            window_id,
            columns,
            rows,
        } => super::program_operations::resize(context, window_id, columns, rows)?,
        _ => unreachable!("program dispatcher received control request"),
    };
    Ok(response)
}
