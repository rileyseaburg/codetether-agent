//! Shared chat and tool-preview scroll methods for native AppState.
pub use codetether_tool_panel::TOOL_PREVIEW_FOLLOW;
codetether_chat_layout::impl_app_chat_scroll!(super::AppState);
codetether_tool_panel::impl_app_tool_scroll!(super::AppState);
