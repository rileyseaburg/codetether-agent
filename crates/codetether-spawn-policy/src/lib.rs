//! Spawn admission by owner-scoped identity and configured nesting depth.
//!
//! [reserve] holds an atomic pending name claim while checking existing live
//! and durable names through [IdentityHost]. Commit the [Reservation] only
//! after durable registration; dropping it releases an unfinished claim.
//! [validate_depth] checks a depth already derived from the live parent.
//! [is_free_or_eligible] and [model_warning] retain model cost admission policy.
pub mod admission;
mod depth;
mod host;
mod identity;
pub use depth::validate as validate_depth;
pub use host::IdentityHost;
pub use identity::{Reservation, reserve};

mod model;
mod model_registry;
mod model_warning;
mod policy_constants;
mod policy_free;
mod policy_parse;
mod policy_registry;
pub use model::is_free_or_eligible;
pub use model_registry::ModelRegistry;
pub use model_warning::for_model as model_warning;
