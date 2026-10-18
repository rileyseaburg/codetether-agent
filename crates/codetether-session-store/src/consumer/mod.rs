//! Consumer snapshot assembly with protected migration and shared hydration.
//!
//! [`next`] reads within the caller's blocking context using an explicit path.
//! [`next_for`] resolves the session location and runs the same read on Tokio's
//! blocking pool. Both return the existing [`crate::Delivery`] contract.
mod next;
mod next_for;
pub use next::next;
pub use next_for::next_for;
