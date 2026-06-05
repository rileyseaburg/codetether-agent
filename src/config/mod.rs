//! Configuration system
//!
//! Handles global, project, and environment-backed settings.

pub mod guardrails;

mod a2a;
mod agent;
mod bool_parse;
mod core;
mod default;
mod env;
mod legacy;
mod load;
mod lsp;
mod merge;
mod merge_sections;
mod path;
mod permission;
mod provider;
mod session;
mod set;
mod telemetry;
mod ui;

pub use a2a::{A2aConfig, AutoApprovePolicy};
pub use agent::AgentConfig;
pub use core::Config;
pub use lsp::{LspLinterEntry, LspServerEntry, LspSettings};
pub use permission::{PermissionAction, PermissionConfig};
pub use provider::ProviderConfig;
pub use session::SessionConfig;
pub use telemetry::TelemetryConfig;
pub use ui::UiConfig;

#[cfg(test)]
mod path_tests;
#[cfg(test)]
mod tests;
