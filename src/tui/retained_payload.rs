//! Shared byte and item bounds for retained view data.
pub use codetether_view_retention::{
    AGENT_OUTPUT_MAX_BYTES, CHAT_EXPANDED_MAX_ITEMS, CHAT_RETAINED_MAX_ITEMS,
    DETAIL_HISTORY_MAX_ITEMS, TOOL_DETAIL_MAX_BYTES, bounded, push_capped,
};
