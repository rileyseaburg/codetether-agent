//! Failed-completion recovery and error presentation order.
//!
//! Implement [Host] for session ownership, retry scheduling and presentation.
//! [apply] restores the session before scheduling recovery and showing the error.
mod host;
mod run;
pub use host::Host;
pub use run::apply;
