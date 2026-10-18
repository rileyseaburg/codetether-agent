//! Swarm integration for agents.
//!
//! This module implements the swarm actor and handler traits so an `Agent` can
//! participate in delegated task execution.
//!
//! # Examples
//!
//! ```ignore
//! let id = agent.actor_id();
//! ```

mod actor_impl;
mod handler_impl;
mod task_execution;
