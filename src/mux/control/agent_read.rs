//! Semantic observation for one mux-backed agent.

use anyhow::Result;

mod session;
mod terminal;
mod text;

/// Read the retained output tail from a named mux agent.
///
/// # Errors
///
/// Returns an error when discovery, authentication, or terminal reading fails.
pub(crate) async fn read_agent_output(name: &str) -> Result<Option<String>> {
    let Some(record) = super::agent_target::load(name).await? else {
        return Ok(None);
    };
    if let Some(output) = session::read(&record).await? {
        return Ok(Some(output));
    }
    terminal::read(name, &record).await.map(Some)
}
