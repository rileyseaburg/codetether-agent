//! Fast-path lookup and refresh of an already resident child.

use crate::{EnsureOpen, Host, ResumeConfig};
use codetether_collaboration_state::residency as state;

pub(super) async fn find<H: Host>(
    target: &str,
    owner: Option<&str>,
    config: &ResumeConfig,
) -> anyhow::Result<Option<EnsureOpen>> {
    let Some((agent_id, mut entry)) = H::resolve(target, owner) else {
        return Ok(None);
    };
    super::refresh::restored::<H>(&agent_id, &mut entry, config).await?;
    state::touch(&agent_id);
    Ok(Some(EnsureOpen::Ready {
        agent_id,
        name: entry.name,
        resumed: false,
    }))
}
