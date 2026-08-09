//! Non-mutating discovery of the retained output tail for SSE consumers.

use crate::mux::protocol::ServerResponse;

use super::context::ServerContext;

pub(super) fn apply(context: &ServerContext, id: u64) -> anyhow::Result<ServerResponse> {
    let tail = context.programs.tail(id)?;
    Ok(ServerResponse::ProgramAttached {
        window_id: id,
        offset: tail.offset,
        replay_until: tail.replay_until,
        alternate_screen: tail.alternate_screen,
    })
}
