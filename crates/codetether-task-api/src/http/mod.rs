//! Queue, agent-task and dispatch HTTP extraction and route assembly.
//!
//! Implement [HttpState] on application state, then merge [queue_routes],
//! [agent_routes] and [dispatch_routes] beneath the existing middleware.
mod agent;
mod agent_routes;
mod dispatch;
mod dispatch_routes;
mod output;
mod queue;
mod queue_routes;
mod state;
pub use agent_routes::agent_routes;
pub use dispatch_routes::dispatch_routes;
pub use queue_routes::queue_routes;
pub use state::HttpState;
