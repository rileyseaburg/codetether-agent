//! Public sub-agent loop with native registry and policy bindings.
mod api;
mod application;
pub use api::run_agent_loop;
pub(crate) use application::Native as LoopHost;
