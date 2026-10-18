//! Local-stage service bindings for an application-bound executor.
//!
//! [execute] retains the planner's providers and passes the same executor
//! services to local scheduling. [LocalStageHost] supplies provider lookup and
//! native job execution; cache and delegation behavior belong to this module.
use crate::SwarmExecutor;
mod cache;
mod cache_binding;
mod execution_binding;
mod host;
mod provider_binding;
mod providers;
mod run;
mod service_binding;
pub use host::LocalStageHost;
pub use run::execute;
struct LocalStage<'a, H: LocalStageHost>(&'a SwarmExecutor<H>);
