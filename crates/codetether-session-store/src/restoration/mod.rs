//! Indexed restoration, consumer hydration and locator recovery.
//!
//! [Restore] supplies native decoding and mutable fields. [load] and
//! [load_sync] preserve snapshot and checkpoint order; [restore_consumer]
//! creates a read-only consumer window inside the caller's transaction.
mod finish_restore;
mod hydrate;
mod load;
mod load_sync;
mod recover_locator;
mod restore;
mod restore_consumer;
mod tail_load;
pub use load::load;
pub use load_sync::load_sync;
pub use restore::Restore;
pub use restore_consumer::restore_consumer;
pub use tail_load::TailLoad;
