//! Existing child lifecycle publications over the shared process bus.
mod result;
mod status;
pub use result::announce_result;
pub use status::{announce_done, announce_working};
