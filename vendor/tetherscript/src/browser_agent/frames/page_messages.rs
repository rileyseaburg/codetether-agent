use super::origin::{frame_origin, target_origin_matches};
use super::{FrameId, FrameMessage};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Queue deterministic metadata for a frame `postMessage` attempt.
    pub fn post_frame_message(
        &mut self,
        source: FrameId,
        target: FrameId,
        data: impl Into<String>,
        target_origin_filter: impl Into<String>,
    ) -> Result<FrameMessage, String> {
        let tree = self.frame_tree();
        let source_origin = frame_origin(self, &tree, source)?;
        let target_origin = frame_origin(self, &tree, target)?;
        let filter = target_origin_filter.into();
        let allowed = self
            .security_policy()
            .allows(&source_origin, &target_origin);
        let delivered = allowed && target_origin_matches(&filter, &target_origin);
        Ok(self.frame_windows.push_message(FrameMessage {
            sequence: 0,
            source,
            target,
            data: data.into(),
            target_origin_filter: filter,
            same_origin: source_origin.is_same_origin(&target_origin),
            source_origin,
            target_origin,
            allowed_by_policy: allowed,
            delivered,
        }))
    }

    /// Return all queued frame message metadata.
    pub fn frame_messages(&self) -> &[FrameMessage] {
        self.frame_windows.messages()
    }

    /// Return queued frame messages targeting `target`.
    pub fn frame_messages_for(&self, target: FrameId) -> Vec<FrameMessage> {
        self.frame_windows.messages_for(target)
    }

    /// Drain queued frame messages targeting `target`.
    pub fn take_frame_messages_for(&mut self, target: FrameId) -> Vec<FrameMessage> {
        self.frame_windows.take_messages_for(target)
    }

    /// Clear all queued frame messages.
    pub fn clear_frame_messages(&mut self) {
        self.frame_windows.clear_messages();
    }
}
