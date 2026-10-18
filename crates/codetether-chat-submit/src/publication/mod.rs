//! Visible user/image messages and missing-provider reporting.
//!
//! [push_user_messages] appends the compact transcript entries. [no_provider]
//! resets native processing state before reporting the configuration error.
mod binding;
mod failure;
mod messages;
pub use failure::{FailureHost, no_provider};
pub use messages::push_user_messages;
