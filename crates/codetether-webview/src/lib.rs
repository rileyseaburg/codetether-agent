//! Webview panel composition and chat presentation.
//!
//! Implement [Host] to provide application-specific panel adapters, then call
//! [render] for a frame. [chat] also exposes transcript and composer renderers.
pub mod chat;
mod host;
mod render;
pub use host::{Composer, Host, Overlays, Panels, State};
pub use render::render;
