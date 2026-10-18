//! Compatibility name for the shared runtime's pause and cancellation handle.
//!
//! [`SwarmControl`] uses the same ownership table as session executors. Clones
//! observe cancellation and pause changes together; the last standalone clone
//! releases its run, while attached handles leave ownership with their session.
//!
//! # Examples
//! ```
//! use codetether_agent::swarm::SwarmControl;
//! let control = SwarmControl::new();
//! control.pause();
//! assert!(control.is_paused());
//! control.resume();
//! assert!(!control.is_paused());
//! control.cancel();
//! assert!(control.is_cancelled());
//! ```

pub use crate::session::runtime::ControlHandle as SwarmControl;
