//! # Search — LLM-routed search across grep, glob, web, memory, and RLM.
//!
//! The router uses `codetether models` discovery (via [`ProviderRegistry`])
//! plus a single LLM call to pick the best backend for a natural-language
//! query, then invokes that backend and returns a normalized
//! [`result::RouterResult`].
//!
//! ## Quick start
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use std::sync::Arc;
//! use codetether_agent::provider::ProviderRegistry;
//! use codetether_agent::search::run_router_search;
//! use codetether_agent::search::model::DEFAULT_ROUTER_MODEL;
//!
//! let registry = Arc::new(ProviderRegistry::from_vault().await.unwrap());
//! let res = run_router_search(registry, DEFAULT_ROUTER_MODEL, "where is fn main", 1).await.unwrap();
//! assert_eq!(res.runs.len(), 1);
//! # });
//! ```

pub mod dispatch;
pub use codetether_application_providers::routing as model;
pub use codetether_application_providers::search as engine;
pub use codetether_search::parse;
pub use codetether_search::prompt;
pub use codetether_search::request;
pub use codetether_search::result;
pub use codetether_search::types;

pub use engine::run_router_search;
pub use result::{BackendRun, RouterResult};
pub use types::{Backend, BackendChoice};
