//! Agent header metadata and formatting owned by the dashboard crate.
pub use codetether_agent_dashboard::bar::tab::{AgentTabMeta, agent_tab};
/// Compatibility export for nested tab styling callers.
pub mod agent_tab_style {
    pub use codetether_agent_dashboard::bar::style::tab_style;
}
