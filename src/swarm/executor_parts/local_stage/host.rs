//! Native services bound to one local-stage executor.
use super::SwarmExecutor;
mod cache;
mod cache_binding;
mod execution_binding;
mod provider_binding;
mod providers;
mod service_binding;
pub(super) struct Native<'a>(pub &'a SwarmExecutor);
