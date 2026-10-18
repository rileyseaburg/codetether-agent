//! Native registry and session service boundaries.
//!
//! Registry operations, session views, mutation and context completion have
//! separate contracts; [Host] combines them for one concrete application.
mod context;
mod registry;
mod session_view;
mod session_write;
pub use context::Context;
pub use registry::Registry;
pub use session_view::SessionView;
pub use session_write::SessionWrite;
/// Native services used by [crate::Agent].
/// # Implementors
/// Bind the four focused interfaces on an application service type.
pub trait Host: Registry + SessionWrite + Context {}
impl<T: Registry + SessionWrite + Context> Host for T {}
