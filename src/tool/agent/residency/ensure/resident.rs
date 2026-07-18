//! Fast-path lookup and refresh of an already resident child.

use super::super::{EnsureOpen, ResumeConfig};

pub(super) async fn find(
    target: &str,
    owner: Option<&str>,
    config: &ResumeConfig,
) -> anyhow::Result<Option<EnsureOpen>> {
    let Some((agent_id, mut entry)) =
        super::super::super::store::scope::resolve_for_parent(target, owner)
    else {
        return Ok(None);
    };
    super::refresh::restored(&agent_id, &mut entry, config).await?;
    super::super::order::touch(&agent_id);
    Ok(Some(EnsureOpen::Ready {
        agent_id,
        name: entry.name,
        resumed: false,
    }))
}
