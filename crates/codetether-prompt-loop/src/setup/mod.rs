//! Ordered model, workspace, and progress initialization for a prompt turn.
//!
//! [initialize] prepares [Parts] through application [Host] bindings. The host
//! retains the borrowed session and provider registry; parts own turn state.
mod host;
mod initialize;
mod parts;
pub use host::Host;
pub use initialize::initialize;
pub use parts::Parts;
