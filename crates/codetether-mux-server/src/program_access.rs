//! Check session ownership before dispatching any program request.
use super::context::ServerContext;
use codetether_mux_protocol::ProgramRequest;

pub(super) async fn check(
    context: &ServerContext,
    session: &str,
    request: &ProgramRequest,
) -> anyhow::Result<()> {
    if let Some(id) = super::program_scope::window_of(request) {
        super::program_scope::owned(context, session, id).await?;
    }
    Ok(())
}
