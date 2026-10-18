//! Worker reconnection sequences for standalone and integrated serving.
//!
//! [run_worker_loop] uses a fixed reconnect delay. [run_worker_server_loop]
//! binds a notification channel and applies shared backoff/breaker policy.
//! Implement [Worker] for native services and [Server] for integrated state.
mod host;
mod server;
mod standalone;
pub use host::{Server, Worker};
pub use server::run_worker_server_loop;
pub use standalone::run_worker_loop;
