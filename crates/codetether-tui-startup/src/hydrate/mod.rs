//! Initial and completed frontend hydration after startup.
//!
//! [initial] and [loading] prepare the first frame. [complete] applies loaded
//! session settings, synchronizes transcript/worker state and presents the
//! session outcome and trust policy in the original startup order.
mod complete;
mod host;
mod initial;
mod network;
mod notice;
pub use complete::complete;
pub use host::{Effects, Fields, Host, Session};
pub use initial::{initial, loading};
