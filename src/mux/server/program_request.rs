//! Mapping PTY protocol requests onto the server-owned registry.

use crate::mux::protocol::{ProgramRequest, ServerResponse};
use crate::mux::pty::TerminalSize;

use super::context::ServerContext;

pub(super) async fn execute(
    context: &ServerContext,
    request: ProgramRequest,
) -> anyhow::Result<ServerResponse> {
    let response = match request {
        ProgramRequest::Start {
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
        ProgramRequest::Attach {
            window_id,
            columns,
            rows,
        } => super::program_operations::attach(context, window_id, columns, rows)?,
        ProgramRequest::Input { window_id, data } => {
            context.programs.input(window_id, &data)?;
            ServerResponse::Acknowledged
        }
        ProgramRequest::Read { window_id, offset } => {
            super::program_operations::read(context, window_id, offset).await?
        }
        ProgramRequest::Resize {
            window_id,
            columns,
            rows,
        } => super::program_operations::resize(context, window_id, columns, rows)?,
    };
    Ok(response)
}
