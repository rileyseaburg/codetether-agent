//! Durable child-agent manifest storage and canonical path lookup.
//!
//! [atomic] reads and writes JSON sidecars, [paths] preserves validated storage
//! locations, and [scan] discovers [Manifest] values without constructing live
//! sessions. [Lifecycle] records the persisted open/closed state.
pub mod atomic;
mod manifest;
pub mod paths;
pub mod scan;
mod tree_target;
pub use manifest::{Lifecycle, Manifest};
