//! Pointer, focus, and scroll operations.

mod focus;
mod pointer;
mod scroll;

/// Focus and blur operations.
pub(in crate::browser::session::native) use focus::{blur, focus};
/// Pointer click and hover operations.
pub(in crate::browser::session::native) use pointer::{click, hover};
/// Scroll operation.
pub(in crate::browser::session::native) use scroll::scroll;
