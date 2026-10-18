//! Windows computer-use action dispatch, desktop capture and native OCR.
//!
//! [dispatch] applies shared routing before selecting native input, capture,
//! or OCR handlers. Shadow input and low-level Windows services come from
//! their dedicated crates. This crate does not manage the isolated worker.
#![cfg(windows)]
use codetether_computer_capture as capture_preview;
use codetether_computer_capture::limits as capture_limits;
use codetether_computer_contract::{input, response, routing};
use codetether_computer_shadow as shadow;
mod platform;
pub use platform::dispatch;
