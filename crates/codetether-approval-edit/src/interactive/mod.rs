//! Approval-backed editor save, cancel and decision delivery.
//!
//! [handle] routes editor actions using a frontend [Host]. [apply] records the
//! edited proposal and delivers the decision to the existing waiting tool.
//! [BorrowedHost] adapts independent frontend fields without generating code.
mod borrowed_host;
mod close;
mod finish;
mod host;
mod key;
pub use borrowed_host::BorrowedHost;
pub use finish::apply;
pub use host::Host;
pub use key::handle;
