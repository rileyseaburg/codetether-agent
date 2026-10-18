//! Mapping PTY protocol requests onto the server-owned registry.
//!
//! Window ids are server-wide, so every request is checked against the
//! bound session before the registry is touched.

use codetether_mux_protocol::{ProgramRequest, ServerResponse};
use codetether_mux_pty::TerminalSize;

use super::context::ServerContext;

pub(super) async fn execute(
    context: &ServerContext,
    session: &str,
    request: ProgramRequest,
) -> anyhow::Result<ServerResponse> {
    super::program_access::check(context, session, &request).await?;
    let response = match request {
        ProgramRequest::Start {
            window_id,
            command,
            columns,
            rows,
        } => {
            let size = TerminalSize::new(columns, rows);
            return super::program_start::start(context, session, window_id, &command, size).await;
        }
        ProgramRequest::Attach {
            window_id,
            columns,
            rows,
        } => super::program_operations::attach(context, window_id, columns, rows)?,
        ProgramRequest::Tail { window_id } => super::program_tail::apply(context, window_id)?,
        ProgramRequest::Input { window_id, data } => {
            context.programs.input(window_id, &data)?;
            ServerResponse::Acknowledged
        }
        ProgramRequest::Steer { text } => {
            super::program_steer::apply(context, session, &text).await?
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
