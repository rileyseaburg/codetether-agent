//! Adapt shared provider requests and responses to RLM's LLM interface.
//!
//! [ProviderWrap] implements [crate::traits::LlmProvider] over a shared
//! provider handle. [build_req] and [to_llm_resp] expose the same conversions
//! for callers that need them separately. No session or cognition state is held.
//!
//! # Examples
//! ```
//! let request = codetether_rlm::provider_adapter::build_req(vec![], vec![], "model", None);
//! assert_eq!(request.model, "model");
//! ```
mod provider;
mod request;
mod response;
pub use provider::ProviderWrap;
pub use request::build_req;
pub use response::to_llm_resp;
