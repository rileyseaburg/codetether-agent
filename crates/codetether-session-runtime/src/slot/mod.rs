//! Move-based session ownership with a retained read-only view.
//! [SessionSlot] holds a session until execution checks it out. [ViewSource]
//! supplies a lightweight snapshot that remains available during checkout.
mod source;
mod state;
pub use source::ViewSource;
pub use state::SessionSlot;
