pub mod app;
pub mod chat;
pub mod ui;
pub mod models;
pub mod utils;
pub mod constants;
pub mod lsp;
pub mod rlm;
pub mod sessions;
pub mod settings;
pub mod help;
pub mod theme;
pub mod terminal;
pub mod status;
pub mod input;

pub mod swarm_view;
pub mod ralph_view;
pub mod theme_utils;
pub mod token_display;
pub mod message_formatter;
pub mod bus_log;
pub mod symbol_search;
pub mod worker_bridge;
pub mod color_palette;

// Simplify public API by re-exporting core types
pub use app::run::run;
pub use app::state::App;
pub use chat::message::{ChatMessage, MessageType};
pub use models::*;

