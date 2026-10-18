//! Paste destination priority and newline filtering for interactive clients.
//! [handle] normalizes text and routes to symbol search, filters or chat.
//! [Host] binds the native view and editing services through [View].
mod chars;
mod host;
mod run;
mod view;
pub use host::Host;
pub use run::handle;
pub use view::View;
