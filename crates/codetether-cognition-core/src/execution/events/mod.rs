//! Check-result events for request rejection or completed tool execution.
//!
//! Constructors preserve the existing payload fields for each outcome.
use crate::{ThoughtEvent, ThoughtEventType};
use chrono::Utc;
use uuid::Uuid;
mod not_allowed;
pub use not_allowed::not_allowed;
mod not_found;
pub use not_found::not_found;
mod invalid_args;
pub use invalid_args::invalid_args;
mod expired;
pub use expired::expired;
mod executed;
pub use executed::executed;
