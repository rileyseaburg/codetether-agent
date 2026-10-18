//! Chat paste routing between images, text sidecars and direct insertion.
//! [paste_into_chat] preserves image-first handling and recursively processes
//! caption text. [Host] supplies native image attachment and composer editing.
mod host;
mod paste;
mod summarise;
pub use host::Host;
pub use paste::paste_into_chat;
