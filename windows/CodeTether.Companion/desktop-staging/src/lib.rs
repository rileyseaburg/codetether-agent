//! Native Windows desktop eligibility and memory-only selected-monitor capture.
//!
//! [`monitors`] supplies immutable choices for local UI selection; retain the
//! chosen [`Monitor`] and call [`validate_selection`] immediately before capture.
//! This check is only a snapshot, not consent or a lock against desktop changes.
//! Hosts must also watch session/display changes, pause, cancel in-flight work,
//! revalidating the shared monitor. [`capture_selected`] returns a bounded JPEG.
//! [`type_in_focused_input`] delivers supplied text through Win32 keyboard input,
//! directly at the existing caret with editable-target checks. Neither path grants
//! consent; callers first obtain local pairing/monitor-sharing authorization.
//! No disk, credentials, inference, or network I/O live here.
//!
//! ```
//! use codetether_companion_desktop::Bounds;
//! let bounds = Bounds::new(-1920, 0, 0, 1080).unwrap();
//! assert!(bounds.contains(-1, 0));
//! assert!(!bounds.contains(0, 0));
//! ```
#![deny(unsafe_op_in_unsafe_fn)]

mod bounds;
mod typing;
mod typing_error;
pub use typing::type_in_focused_input;
pub use typing_error::TypingError;
mod capture;
mod error;
mod frame;
mod monitor;
mod platform;
mod selection;
pub use bounds::Bounds;
pub use capture::capture_selected;
pub use error::Error;
pub use frame::CapturedFrame;
pub use monitor::Monitor;
pub use platform::{check_available, monitors};
pub use selection::validate_selection;
