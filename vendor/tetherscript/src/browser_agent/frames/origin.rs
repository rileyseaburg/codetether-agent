use super::{FrameId, FrameTree};
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::Origin;

pub(super) fn frame_origin(
    page: &BrowserPage,
    tree: &FrameTree,
    id: FrameId,
) -> Result<Origin, String> {
    let frame = tree
        .frame(id)
        .ok_or_else(|| format!("unknown frame {}", id.get()))?;
    if id == tree.root_id() || frame.url() == "about:blank" {
        return Ok(page.current_origin());
    }
    Ok(page.request_security_metadata(frame.url()).target_origin)
}

pub(super) fn target_origin_matches(filter: &str, target: &Origin) -> bool {
    if filter.trim() == "*" {
        return true;
    }
    Origin::parse(filter).is_same_origin(target)
}
