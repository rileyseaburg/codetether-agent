//! Network diagnostics and log readers.

mod diagnose;
mod log;

/// Network diagnostic handler.
pub(in crate::browser::session::native) use diagnose::diagnose;
/// Network log handler.
pub(in crate::browser::session::native) use log::log;
