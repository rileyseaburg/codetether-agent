//! Two-phase initialization of shared tool registries.
//!
//! Implement [Host] for native registry bindings. [with_defaults] and
//! [with_provider] register batch before sharing the registry, then attach a
//! weak reference so batch execution cannot form a strong ownership cycle.
mod host;
mod shared;
pub use host::Host;
pub use shared::{with_defaults, with_provider};
