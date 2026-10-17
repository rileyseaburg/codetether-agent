//! OpenAI-compatible thinker transport with bounded transient retries.
//!
//! [think] sends non-streaming requests using a caller-provided client and
//! configuration. [HttpObserver] keeps logging under host control.
use super::{ThinkerConfig, ThinkerOutput};
mod observer;
mod pace;
mod response;
pub mod retry;
mod run;
mod send;
mod status;
pub use observer::HttpObserver;
pub use run::think;
