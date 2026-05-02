pub mod app;
pub mod chat;
pub mod clipboard;
pub mod clipboard_ssh;
pub mod clipboard_text;
#[cfg(windows)]
pub mod clipboard_winapi;
pub mod constants;
pub mod help;
pub mod input;
pub mod latency;
pub mod lsp;
pub mod model_picker;
pub mod models;
#[path = "protocol_registry_view.rs"]
pub mod protocol_registry_view;
pub mod rlm;
pub mod sessions;
pub mod settings;
pub mod status;
pub mod terminal;
pub mod theme;
pub mod ui;
pub mod utils;

pub mod agent_identity;
pub mod audit_view;
pub mod bus_log;
pub mod color_palette;
pub mod message_formatter;
pub mod ralph_view;
pub mod swarm_view;
pub mod symbol_search;
pub mod theme_utils;
pub mod token_display;
pub mod worker_bridge;

// Simplify public API by re-exporting the main entrypoint.
pub use app::run::run;
