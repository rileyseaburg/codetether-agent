//! Default byte and item limits for retained terminal view data.

/// Retention budget for tool detail max bytes.
pub const TOOL_DETAIL_MAX_BYTES: usize = 8 * 1024;

/// Retention budget for agent output max bytes.
pub const AGENT_OUTPUT_MAX_BYTES: usize = 16 * 1024;

/// Retention budget for detail history max items.
pub const DETAIL_HISTORY_MAX_ITEMS: usize = 100;

/// Retention budget for chat retained max items.
pub const CHAT_RETAINED_MAX_ITEMS: usize = 500;

/// Retention budget for chat expanded max items.
pub const CHAT_EXPANDED_MAX_ITEMS: usize = 2_000;
