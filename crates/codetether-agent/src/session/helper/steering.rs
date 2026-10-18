//! Compatibility path for session steering; implementation belongs to the runtime.

pub use crate::session::runtime::steering::*;
pub(crate) use crate::session::runtime::steering::{SteeringInput, clear, drain_into, open, push};
