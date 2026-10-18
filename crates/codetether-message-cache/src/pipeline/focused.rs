//! Focused transcripts take precedence over the main-chat cache.
use super::Host;
use crate::DrawnLines;
pub(super) fn build(host: &mut impl Host, content_width: usize) -> Option<DrawnLines> {
    if let Some(lines) = host.swarm_lines(content_width) {
        return Some(lines);
    }
    host.agent_lines(content_width)
}
