//! Persistent introduction identities using host-selected paths.
//!
//! Loading reports I/O errors; recording is best effort and avoids overwriting
//! a ledger it cannot read. Writes retain the temp-file rename strategy.
mod atomic;
mod load;
mod record;
pub use load::load;
pub use record::record;

mod contains;
pub use contains::contains;
