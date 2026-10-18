// Extra chat-view sub-modules added in this session.
// Included from `mod.rs` via `include!`.
pub(crate) mod agent_lines;
pub use codetether_agent_dashboard::bar::style as agent_tab_style;
pub(crate) mod approval_diff;
pub(crate) mod approval_overlay_content;
pub use codetether_message_bubbles as bubble_mods;
pub use codetether_message_bubbles::bubble_assistant;
pub use codetether_token_display::context_gauge::context_gauge_heat;
pub(crate) mod input_title;
#[cfg(test)]
#[path = "input_title_tests.rs"]
mod input_title_tests;
pub use codetether_chat_layout::status_glue;
pub use codetether_chat_status::kb_style;
pub use codetether_composer_view as suggestion_style;
pub use codetether_tui_style::spinner::spinner_hue;
pub mod suggestions;
pub(crate) mod swarm_messages;
pub use codetether_message_bubbles::timeline;
pub use codetether_tui_style::thinking_pulse;
pub mod title;
pub use codetether_chat_status::title_brand;
pub mod title_labels;
pub(crate) use codetether_chat_layout as viewport;

mod composer_view;
