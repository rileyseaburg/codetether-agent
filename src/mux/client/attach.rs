//! Interactive attachment to one session on a persistent mux server.

use anyhow::Result;

use crate::mux::protocol::ClientRequest;
use crate::mux::registry::SessionTarget;

use super::connection::MuxConnection;
use super::state::ClientState;

pub(in crate::mux) async fn attach(target: &SessionTarget) -> Result<()> {
    let mut connection = MuxConnection::connect(target).await?;
    super::render::help();
    let response = connection.request(ClientRequest::Snapshot).await?;
    let mut state = ClientState::new(&target.session);
    state.update(&response);
    super::render::response(&response, &state.session);
    let active = state.active_id()?;
    if let Some(outcome) = super::program::attach(&mut connection, active).await? {
        if outcome == super::proxy::Outcome::Detached {
            let _ = connection.request(ClientRequest::Detach).await;
            return Ok(());
        }
        println!("\nprogram exited");
    }
    super::session::run(&mut connection, state).await
}
