//! Symbol mentions for a host-owned text composer.
//!
//! Implement [Composer] with the application's editing operations. [open]
//! starts a word-initial mention; [confirm] attaches the selected source symbol;
//! [dismiss] preserves the typed query. [starts_mention] checks the boundary.
mod boundary;
mod composer_view;
mod confirm;
mod dismiss;
mod host;
pub use composer_view::ComposerView;
mod open;
pub use boundary::starts_mention;
pub use confirm::confirm;
pub use dismiss::dismiss;
pub use host::Composer;
pub use open::open;
#[cfg(test)]
mod boundary_tests;

mod selection;
pub use selection::confirm_selection;
