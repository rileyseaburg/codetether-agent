//! Control and program outcomes for the mux command prompt.

use anyhow::Result;

use codetether_mux_protocol::ClientRequest;

use crate::Outcome;
use codetether_mux_client::ClientState;
use codetether_mux_client::MuxConnection;

pub(super) async fn finish_program(connection: &mut MuxConnection, outcome: Outcome) -> bool {
    if outcome == Outcome::Detached {
        let _ = connection.request(ClientRequest::Detach).await;
        true
    } else {
        println!("\nprogram exited");
        false
    }
}

/// Request detachment, ignoring failures as in the original control loop.
/// # Arguments
/// `connection` is the authenticated transport.
/// # Returns
/// Unit after attempting the detach request.
/// # Examples
/// ```no_run
/// # async fn example(connection: &mut codetether_mux_client::MuxConnection) {
/// codetether_mux_attachment::detach(connection).await;
/// # }
/// ```
pub async fn detach(connection: &mut MuxConnection) {
    let _ = connection.request(ClientRequest::Detach).await;
}

pub(super) async fn control(
    connection: &mut MuxConnection,
    state: &mut ClientState,
    request: ClientRequest,
) -> Result<bool> {
    let response = connection.request(request).await?;
    state.update(&response);
    Ok(codetether_mux_client::render::response(
        &response,
        &state.session,
    ))
}
