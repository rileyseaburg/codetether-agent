//! Regression coverage for tool-preview follow and clamping.
use crate::{TOOL_PREVIEW_FOLLOW, ToolPreviewScrollState};

#[test]
fn tool_preview_scroll_auto_follow_and_clamp() {
    let mut state = ToolPreviewScrollState::default();
    // Setting max_scroll while in auto-follow preserves the follow sentinel.
    state.reset_tool_preview_scroll();
    state.set_tool_preview_max_scroll(4);
    assert_eq!(state.tool_preview_scroll, TOOL_PREVIEW_FOLLOW);
    // Scrolling down past max re-enables auto-follow sentinel.
    state.tool_preview_scroll = 3;
    state.scroll_tool_preview_down(10);
    assert_eq!(state.tool_preview_scroll, TOOL_PREVIEW_FOLLOW);
    // Scrolling up cancels auto-follow, starts from current max_scroll.
    state.scroll_tool_preview_up(2);
    assert_eq!(state.tool_preview_scroll, 2);
    // Reducing max_scroll clamps.
    state.set_tool_preview_max_scroll(1);
    assert_eq!(state.tool_preview_scroll, 1);
}
