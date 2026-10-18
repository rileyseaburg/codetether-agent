//! Interactive attachment to one session on a persistent mux server.

use anyhow::Result;

use codetether_mux_protocol::ClientRequest;
use codetether_mux_registry::SessionTarget;

use codetether_mux_client::{ClientState, MuxConnection};

/// Connect to a persistent session and run its interactive terminal and prompt.
///
/// # Arguments
/// * `target` - Session and authenticated server record to connect to.
/// * `clipboard` - Lazy platform clipboard lookup used for paste input.
/// # Returns
/// Returns after the user detaches or closes the prompt.
/// # Errors
/// Returns connection, protocol, terminal or prompt errors.
/// # Examples
/// ```no_run
/// # async fn example(target: &codetether_mux_registry::SessionTarget) -> anyhow::Result<()> {
/// codetether_mux_attachment::attach_session(target, || None).await
/// # }
/// ```
pub async fn attach_session(
    target: &SessionTarget,
    clipboard: impl Fn() -> Option<String>,
) -> Result<()> {
    let mut connection = MuxConnection::connect(target).await?;
    codetether_mux_client::render::help();
    let response = connection.request(ClientRequest::Snapshot).await?;
    let mut state = ClientState::new(&target.session);
    state.update(&response);
    codetether_mux_client::render::response(&response, &state.session);
    let active = state.active_id()?;
    if let Some(outcome) = crate::program::attach(&mut connection, active, &clipboard).await? {
        if outcome == crate::Outcome::Detached {
            let _ = connection.request(ClientRequest::Detach).await;
            return Ok(());
        }
        println!("\nprogram exited");
    }
    crate::session::run(&mut connection, state, clipboard).await
}
