//! Latency inspector rendering independent of application state.
//!
//! [LatencyView] supplies request and tool measurements, [render_latency] draws
//! the inspector, and [push_chat_turn_latency] can append the chat-turn section.
mod chat;
mod completion;
mod current;
mod footer;
mod format;
mod header;
mod last_request;
mod last_tool;
mod providers;
mod recent;
mod recent_rows;
mod render;
mod tool_records;
mod tool_stats;
mod tool_summary;
mod tools;
pub mod transport;
mod view;
pub use chat::push_chat_turn_latency;
pub use render::render_latency;
pub use view::{LatencyView, ToolLatency};
