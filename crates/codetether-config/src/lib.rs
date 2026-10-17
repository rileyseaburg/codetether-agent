//! Configuration loading, policy precedence, and workspace trust.
//!
//! [`Config`] combines serialized settings with global/project loading and
//! explicit runtime overrides. Theme data comes from `codetether-theme`;
//! this crate requires no TUI or terminal renderer.
//!
//! # Examples
//! ```
//! use codetether_config::{AccessMode, Config};
//! let mut config = Config::default();
//! config.apply_access_mode_override(Some(AccessMode::Ask));
//! assert_eq!(config.effective_access_mode(), Some(AccessMode::Ask));
//! ```

mod config;
pub use config::*;
