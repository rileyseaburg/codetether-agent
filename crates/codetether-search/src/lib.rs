//! Search planning and execution over caller-provided model and backend adapters.
//!
//! [run_router_search] asks the model for a plan, executes choices through
//! [BackendExecutor], and returns normalized [RouterResult] values. The caller
//! owns model resolution and the concrete tools. [arguments] supplies backend
//! argument defaults; [parse] and [request] expose plan conversion helpers.
pub mod arguments;
mod backend;
mod collect;
pub mod engine;
pub mod parse;
pub mod prompt;
pub mod request;
pub mod result;
pub mod types;
pub use backend::BackendExecutor;
pub use engine::run_router_search;
pub use result::{BackendRun, RouterResult};
pub use types::{Backend, BackendChoice};

/// Native filename and content search backends.
pub mod native;
