//! Application callback for choosing the provider/model used by a job.
use codetether_provider_core::Provider;
use codetether_rlm::{RlmConfig, RlmModelPurpose};
use std::{future::Future, pin::Pin, sync::Arc};
/// An asynchronous model resolution result with borrowed configuration lifetime.
pub type Resolution<'a> = Pin<Box<dyn Future<Output = (Arc<dyn Provider>, String)> + 'a>>;
/// Native model-selection callback used immediately before a background request.
pub type Resolver =
    for<'a> fn(Arc<dyn Provider>, &'a str, &'a RlmConfig, RlmModelPurpose) -> Resolution<'a>;
