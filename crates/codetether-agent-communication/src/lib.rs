//! Same-turn steering and durable messages for idle child agents.
//!
//! Implement [Host] with native session and steering access. [steer] waits for a
//! starting child to accept input or become idle; [queue_only] then persists a
//! user message without starting another turn. Both share the runtime's existing
//! execution notifications and status channels.
mod active;
mod host;
mod idle;
mod route;
mod wait;
pub use active::steer;
pub use host::Host;
pub use idle::queue_only;
pub use route::Route;
