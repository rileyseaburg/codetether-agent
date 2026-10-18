//! Session status reports for status and permissions utility commands.
//!
//! [show] refreshes workspace trust policy, formats a [Snapshot], and publishes
//! the report through [View]. Native code retains field and message access.
mod lines;
mod policy;
mod show;
mod view;
pub use show::show;
pub use view::{Snapshot, View};
