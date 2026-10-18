//! Persistent subprocess transport for isolated computer-use requests.
//!
//! [execute] serializes parent requests and owns failed child cleanup.
//! [serve_with] supplies the child protocol loop; the host provides direct
//! platform dispatch. Failed exchanges are never replayed automatically.
use codetether_computer_contract::input;
mod worker;
pub use worker::{execute, serve_with};
