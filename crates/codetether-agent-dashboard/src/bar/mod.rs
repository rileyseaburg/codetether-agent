//! Unified agent header tabs and identity styling.
//!
//! Implement [Host] to expose native registries, then [render] the main, local,
//! tool and swarm tabs. [visible] uses the same registry-presence policy.
mod host;
mod presence;
mod render;
pub mod style;
mod swarm;
pub mod tab;
mod tool;
mod tui;
pub use host::{Host, SessionId};
pub use presence::visible;
pub use render::render;
pub use tool::push_tool_agents;
pub use tui::push_tui_agents;
