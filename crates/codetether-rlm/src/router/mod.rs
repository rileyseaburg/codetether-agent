//! RLM Router — decides when and how to route content through RLM.
//!
//! Splits routing decisions, smart truncation, and the iterative
//! auto-processing loop across focused submodules.

mod auto_loop;
mod auto_prepare;
mod auto_process;
mod auto_process_bus;
mod auto_process_emit;
mod extract;
mod fallback;
mod fallback_context;
mod host;
mod legacy_answer;
mod legacy_process;
mod prompts;
mod prompts_system;
mod should_route;
mod truncate;
mod types;

pub use auto_process::auto_process;
pub use extract::extract_final;
pub use fallback::fallback_result;
pub use host::{HostToolResult, RouterHost};
pub use should_route::should_route;
pub use truncate::smart_truncate;
pub use types::{
    CrateAutoProcessContext, IntoCrateCtx, ProcessProgress, RoutingContext, RoutingResult,
};

#[cfg(test)]
mod tests;
