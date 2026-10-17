//! Bash sandbox configuration and policy construction.
//!
//! The [config] module resolves configuration and approval-aware decisions,
//! [mode] reads environment defaults, and [policy] constructs execution limits.
//! Shared runtime policy owns approval state; this crate owns no duplicate state.
//!
//! ```rust
//! let root = codetether_bash_policy::mode::root(Some("/workspace".into()));
//! assert_eq!(root, std::path::PathBuf::from("/workspace"));
//! ```

pub mod config;
pub mod mode;
pub mod policy;
