//! Semantic observation for one mux-backed agent.

use anyhow::Result;

mod session;
mod terminal;
mod text;

/// Read the retained output tail from a named mux agent.
///
/// # Arguments
/// * `name` - Mux session route ID.
/// * `reader` - Native indexed-session projection reader.
/// # Returns
/// Semantic transcript output or a bounded terminal tail, if the session exists.
/// # Examples
/// ```text
/// read_agent_output("work", &projection_reader).await?
/// ```
/// # Errors
///
/// Returns an error when discovery, authentication, or terminal reading fails.
pub async fn read_agent_output(
    name: &str,
    reader: &impl ProjectionReader,
) -> Result<Option<String>> {
    let Some(target) = super::target::load(name).await? else {
        return Ok(None);
    };
    if let Some(output) = session::read(&target, reader).await? {
        return Ok(Some(output));
    }
    terminal::read(name, &target).await.map(Some)
}

/// Indexed-session projection access supplied by the host.
///
/// # Implementors
/// Hosts resolve storage paths and read the requested materialized view.
pub trait ProjectionReader: Sync {
    /// Read a named view, returning None when no indexed view is available.
    /// # Errors
    /// Propagates storage-path and projection-read errors.
    fn read(
        &self,
        id: &str,
        name: &str,
    ) -> impl Future<Output = Result<Option<serde_json::Value>>> + Send;
}
