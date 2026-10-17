//! Build worker registration metadata from shared catalogs and host settings.
//!
//! Model/agent projection is separate from provider discovery and the HTTP
//! registration request. Host metadata preserves the worker environment policy.
mod agents;
mod host;
mod models;
pub use agents::builtin_agent_defs;
pub use host::host_metadata;
pub use models::build_models_array;
