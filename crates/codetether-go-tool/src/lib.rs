//! Autonomous task-tool lifecycle and persisted OKR reporting.
//!
//! [`GoTool`] owns validation, provider selection, background state, callbacks,
//! and results. Implement [`Host`] and [`Registry`] to bind an existing provider
//! registry and Ralph executor. Active progress uses `codetether-go-monitor`.

mod background;
mod callback;
mod completion;
mod description;
mod dispatch;
mod failure;
mod host;
mod launch;
mod launch_info;
mod launch_log;
mod launch_response;
mod params;
mod provider;
mod provider_explicit;
mod record;
mod registry;
mod schema;
mod setup;
mod status;
mod status_render;
mod success;
mod template;
mod tool;
mod tool_impl;
mod validation;
pub use completion::Completion;
pub use host::Host;
pub use registry::Registry;
pub use tool::GoTool;
