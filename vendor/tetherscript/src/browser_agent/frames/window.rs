use super::FrameId;

/// Page opener identity for a top-level browsing context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowOpener {
    /// Index of the opener page inside its [`BrowserContext`](crate::browser_agent::BrowserContext).
    pub page_index: usize,
    /// Opener frame, normally the opener page root frame.
    pub frame_id: FrameId,
}

/// Parent, top, and opener metadata for a frame window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameWindowRelation {
    /// Frame this relation describes.
    pub frame_id: FrameId,
    /// Parent frame, or `None` for the page root frame.
    pub parent: Option<FrameId>,
    /// Top/root frame in the current page.
    pub top: FrameId,
    /// Opener metadata for top-level pages opened by another page.
    pub opener: Option<WindowOpener>,
}
