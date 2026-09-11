//! Control and program outcomes for the mux command prompt.

use anyhow::Result;

use crate::mux::protocol::ClientRequest;

use super::super::state::ClientState;
use super::super::{connection::MuxConnection, proxy::Outcome};

pub(super) async fn finish_program(connection: &mut MuxConnection, outcome: Outcome) -> bool {
    if outcome == Outcome::Detached {
        let _ = connection.request(ClientRequest::Detach).await;
        true
    } else {
        println!("\nprogram exited");
        false
    }
}

pub(super) async fn detach(connection: &mut MuxConnection) {
    let _ = connection.request(ClientRequest::Detach).await;
}

pub(super) async fn control(
    connection: &mut MuxConnection,
    state: &mut ClientState,
    request: ClientRequest,
) -> Result<bool> {
    let response = connection.request(request).await?;
    state.update(&response);
    Ok(super::super::render::response(&response, &state.session))
}
