//! HTTP discovery of registered plugins and server signing availability.
//!
//! list_plugins projects a caller-owned registry without changing registration.
pub mod http;
mod response;
mod service;
pub use response::PluginListResponse;
pub use service::list_plugins;
