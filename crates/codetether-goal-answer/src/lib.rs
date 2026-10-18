//! Human question interruption, held-answer delivery and explicit continuation.
//!
//! [intercept] routes explicit questions and continuation input. [deliver]
//! answers a held question only after the native session is available. The
//! application supplies journal operations through [Journal], user-interface
//! actions through [Input], and answer generation through [Delivery].
mod agentic;
mod answer;
pub mod continuation;
mod frontend;
mod host;
mod question;
pub mod release;
mod review;
mod submit;
pub use answer::deliver;
pub use host::{Delivery, Input, InputActions, InputView, Journal};
pub use submit::intercept;
