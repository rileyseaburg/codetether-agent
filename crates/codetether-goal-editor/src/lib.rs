//! In-memory goal editing with conflict-aware, user-authorized saves.
//!
//! [open] creates a virtual editor draft. [handle] and [paste] use the shared
//! editor engine; [close] discards without touching the journal. [Draft] guards
//! semantic goal fields while allowing live usage accounting to advance.
mod draft;
mod frontend;
mod host;
mod keys;
mod open;
mod paste;
mod save;
pub use draft::Draft;
pub use host::{Host, Journal, Read, Session, Write};
pub use keys::handle;
pub use open::{close, open};
pub use paste::paste;
