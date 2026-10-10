//! Deterministic, memory-only capture scheduling; no OS capture or networking.
//!
//! [`Schedule`] starts paused. Only a local Start/resume action after verifying
//! live pairing and selecting a monitor may call [`Schedule::resume`]. The host
//! must serialize access, supply monotonic elapsed time from one fixed epoch,
//! check desktop availability before capture, and pause on lock/disconnect.
//! Pause/revocation must also cancel in-flight capture/upload in the host; this
//! policy cannot cancel I/O and does not itself authenticate remote callers.
//!
//! ```
//! use codetether_companion_capture::{Options, Schedule};
//! use codetether_companion_protocol::CaptureTrigger;
//! use std::time::Duration;
//! let mut schedule = Schedule::default();
//! assert_eq!(schedule.due(Duration::ZERO), None);
//! schedule.resume(Options::new(15, true, false, false).unwrap());
//! assert_eq!(schedule.due(Duration::ZERO), Some(CaptureTrigger::Periodic));
//! schedule.pause();
//! assert_eq!(schedule.due(Duration::ZERO), None);
//! ```
mod due;
mod lifecycle;
mod options;
mod queue;
mod schedule;

pub use options::Options;
pub use schedule::Schedule;
