//! Spawn admission by owner-scoped identity and configured nesting depth.
//!
//! [reserve] holds an atomic pending name claim while checking existing live
//! and durable names through [IdentityHost]. Commit the [Reservation] only
//! after durable registration; dropping it releases an unfinished claim.
//! [validate_depth] checks a depth already derived from the live parent.
mod depth;
mod host;
mod identity;
pub use depth::validate as validate_depth;
pub use host::IdentityHost;
pub use identity::{Reservation, reserve};
