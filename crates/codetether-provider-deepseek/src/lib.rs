//! DeepSeek chat completion and reasoning/tool-call round trips.
//!
//! [`DeepSeekProvider`] implements [`codetether_provider_core::Provider`].
//! Request, response, catalog, and transport modules each own one part of
//! the protocol. Construction reuses the shared provider HTTP pool.
//!
//! ```no_run
//! use codetether_provider_deepseek::DeepSeekProvider;
//! let provider = DeepSeekProvider::with_base_url(
//!     String::new(), "https://api.deepseek.com/v1".into(),
//! ).unwrap();
//! assert!(provider.base_url.starts_with("https://"));
//! ```
mod catalog;
mod provider_impl;
mod request;
mod response;
mod state;
mod transport;
pub use state::DeepSeekProvider;
