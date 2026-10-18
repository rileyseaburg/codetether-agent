//! WebSocket framing and event transport for one authenticated session turn.
//!
//! The host owns session loading and prompt execution. Transport accepts the
//! shared SessionEvent snapshot parameter and preserves ordered thread events.
pub mod event_forward;
pub mod frames;
pub mod handshake;
pub mod send;
pub mod socket;
#[cfg(test)]
mod tests;
pub mod turn_event;
pub mod turn_finish;
pub mod turn_input;
pub mod wire;

mod connection;
mod prompt_run;
mod turn;
mod turn_start;
mod turn_step;
pub use connection::run;
pub use prompt_run::{PromptOutcome, PromptRun, PromptStarter};
