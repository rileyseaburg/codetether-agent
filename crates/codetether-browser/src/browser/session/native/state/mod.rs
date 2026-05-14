//! Send-friendly native backend state.

mod page;
mod runtime;

/// Send-friendly native page state.
pub(in crate::browser::session) use page::NativePage;
/// Send-friendly native runtime state.
pub(in crate::browser::session) use runtime::NativeRuntime;
