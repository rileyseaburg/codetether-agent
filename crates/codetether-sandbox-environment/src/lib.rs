//! Restricted child environments and read-only host toolchain discovery.
//!
//! [restricted] constructs environment variables from sandbox policy.
//! [toolchain] locates allowed runtime roots and preserves visible PATH order.
pub mod environment;
pub mod toolchain;
pub use environment::{restricted, rustup_home};
pub use toolchain::{ENV, path_entries, roots, roots_from};
