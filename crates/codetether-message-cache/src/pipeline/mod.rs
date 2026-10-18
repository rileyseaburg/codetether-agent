//! Focus selection and cached chat-line assembly.
//!
//! Implement [Host] for native transcript rendering, then call [build] each
//! frame. Restore returned [crate::DrawnLines] after drawing to return ownership.
mod build;
mod focused;
mod host;
mod store;
pub use build::build;
pub use host::Host;
