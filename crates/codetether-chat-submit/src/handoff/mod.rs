//! Owned session handoff, steering reservation and failed-transfer recovery.
//!
//! [prepare] captures native execution settings and creates a request. [submit]
//! reserves steering, transfers ownership and restores rejected sessions.
//! [Preparation] and [Transfer] retain concrete runtime types outside this crate.
mod binding;
mod preparation;
mod prepare;
mod submit;
mod transfer;
pub use preparation::Preparation;
pub use prepare::prepare;
pub use submit::submit;
pub use transfer::Transfer;
