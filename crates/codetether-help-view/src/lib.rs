//! Help reference content, scrolling and terminal overlay rendering.
//!
//! Supply [HelpContext] to [build_help_lines], then render the lines with
//! [render_help_overlay] and [HelpScrollState]. [centered_rect] computes popup bounds.
mod agents;
mod blank;
mod build;
mod bus;
mod centered_rect;
mod cmd_row;
mod commands;
mod context;
mod conversation_commands;
mod copy_shortcuts;
mod development;
mod heading;
mod images;
mod key_row;
mod keyboard;
mod navigation_commands;
mod observability;
mod render;
mod scroll;
mod scrolling;
mod separator;
mod session_auto_apply;
mod session_colors;
mod session_completion;
mod session_identity;
mod session_network;
mod session_picker;
mod settings;
mod text_editing;
mod title;
mod tokens;
mod worker;
pub use build::build_help_lines;
pub use centered_rect::centered_rect;
pub use context::HelpContext;
pub use render::render_help_overlay;
pub use scroll::HelpScrollState;
mod state;
pub use state::HelpState;
