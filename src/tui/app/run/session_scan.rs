use std::path::Path;

use crate::session::TailLoad;

/// Selects the startup session policy for the TUI.
///
/// TUI launches always begin with a new session. Existing sessions remain on
/// disk and can still be opened explicitly from the session browser.
pub(super) async fn load(_cwd: &Path) -> anyhow::Result<TailLoad> {
    Err(anyhow::anyhow!("requested fresh session"))
}
