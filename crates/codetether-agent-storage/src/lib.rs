//! Durable child-agent manifest storage and canonical path lookup.
//!
//! [atomic] reads and writes JSON sidecars, [paths] preserves validated storage
//! locations, and [scan] discovers [Manifest] values without constructing live
//! sessions. [Lifecycle] records the persisted open/closed state.
//!
//! Implement [Host] to bind session loading, registry changes and mailbox effects.
//! Call [hydrate_parent] for startup recovery, [prepare_resume] and [activate_resume]
//! for a closed child, and [reparent_owner] for continuation ownership transfer.
pub mod atomic;
mod manifest;
pub mod paths;
pub mod scan;
mod tree_target;
pub use manifest::{Lifecycle, Manifest};

mod activate;
mod close;
mod closed;
mod entry_manifest;
mod host;
mod hydrate;
mod lookup;
mod manifest_io;
mod migration;
mod prepare;
mod restore;
mod runtime_host;
pub use activate::activate_resume;
pub use close::{close, exists};
pub use closed::ClosedAgent;
pub use host::Host;
pub use hydrate::hydrate_parent;
pub use lookup::{child_ids, durable_id};
pub use manifest_io::{remove, save, save_lifecycle};
pub use migration::reparent_owner;
pub use prepare::prepare_resume;
pub use runtime_host::RuntimeHost;
