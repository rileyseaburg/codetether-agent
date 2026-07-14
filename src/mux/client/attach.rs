//! Interactive attachment to a persistent mux server.

use anyhow::Result;

use crate::mux::protocol::ClientRequest;
use crate::mux::registry::MuxRecord;

use super::connection::MuxConnection;

pub(in crate::mux) async fn attach(record: &MuxRecord) -> Result<()> {
    let mut connection = MuxConnection::connect(record).await?;
    super::render::help();
    let response = connection.request(ClientRequest::Snapshot).await?;
    let mut state = None;
    super::state::update(&mut state, &response);
    super::render::response(&response);
    let active = super::state::active_id(&state)?;
    if let Some(outcome) = super::program::attach(&mut connection, active).await? {
        if outcome == super::proxy::Outcome::Detached {
            let _ = connection.request(ClientRequest::Detach).await;
            return Ok(());
        }
        println!("\nprogram exited");
    }
    super::session::run(&mut connection, state).await
}
