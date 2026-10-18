//! Search CLI option handling and result presentation.
//!
//! [execute] receives a configured provider registry and embedding policy,
//! resolves the requested router model, invokes shared search and prints output.
//! [render_human] formats an existing result without invoking any backend.
mod args;
mod execute;
mod render;
pub use args::SearchArgs;
pub use execute::execute;
pub use render::render_human;
