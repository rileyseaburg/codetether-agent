//! Protected legacy migration and revision-aware external session imports.
//!
//! Implement [Legacy] on the native session. [ensure] preserves and verifies a
//! legacy JSON file before publishing a locator. [persist] uses the current
//! indexed revision when importing a newer external snapshot.
mod contract;
mod digest;
mod ensure;
mod persist;
pub use contract::Legacy;
pub use ensure::ensure;
pub use persist::persist;
