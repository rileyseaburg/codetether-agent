//! Command transport with direct cancellation and active-turn steering.
//! [SessionHandle] sends owned requests and control messages. [Control]
//! supplies native active-turn reservation, cancellation and steering policy.
mod control;
mod signals;
mod state;
mod steering;
mod submit;
pub use control::Control;
pub use state::SessionHandle;
