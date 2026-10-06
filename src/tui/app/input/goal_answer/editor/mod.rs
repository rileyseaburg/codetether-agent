//! In-memory goal editor: draft text is separate from the live goal until saved.

mod draft;
mod keys;
mod open;
mod paste;
mod save;
pub(crate) use draft::Draft;
pub(crate) use keys::handle;
pub(crate) use open::{close, open};
pub(crate) use paste::paste;

#[cfg(test)]
mod tests;
