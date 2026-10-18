//! Apply program start/attach outcomes to the interactive control loop.
use anyhow::Result;
use codetether_mux_client::{ClientState, MuxConnection};
pub(super) async fn start(
    connection: &mut MuxConnection,
    state: &ClientState,
    command: String,
    clipboard: impl Fn() -> Option<String>,
) -> Result<bool> {
    let id = state.active_id()?;
    let outcome = crate::program::start(connection, id, command, clipboard).await?;
    Ok(crate::control::finish_program(connection, outcome).await)
}
pub(super) async fn attach(
    connection: &mut MuxConnection,
    state: &ClientState,
    clipboard: impl Fn() -> Option<String>,
) -> Result<bool> {
    let id = state.active_id()?;
    Ok(
        match crate::program::attach(connection, id, clipboard).await? {
            Some(outcome) => crate::control::finish_program(connection, outcome).await,
            None => {
                eprintln!("mux: active window has no running program");
                false
            }
        },
    )
}
