//! Combine application indexed search with shared RLM answer synthesis.
//!
//! [`ApplicationHost`] delegates retrieval to [`Search`] and preserves the
//! session-recall tool's synthesis context and application bus specialization.
use std::marker::PhantomData;
mod context;
mod host;
mod search;
pub use search::Search;
/// Recall host using bus type `B` and indexed-search adapter `S`.
/// # Examples
/// ```no_run
/// # fn example<B: codetether_rlm_routing::Bus, S: codetether_session_recall::application::Search>() {
/// fn accepts_host<H: codetether_session_recall::Host>() {}
/// accepts_host::<codetether_session_recall::application::ApplicationHost<B, S>>();
/// # }
/// ```
pub struct ApplicationHost<B, S>(PhantomData<fn() -> (B, S)>);
