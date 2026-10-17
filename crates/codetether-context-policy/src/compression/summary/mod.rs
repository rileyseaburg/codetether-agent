//! Shared compressed-history messages and dropped-range navigation.
//!
//! [install] replaces history with a summary followed by its retained tail.
//! [wrap_cached] preserves the original request prefix in an oversized-message
//! summary; [toc::render_toc] supplies a navigable index of dropped user turns.
mod cached;
mod install;
pub mod toc;
pub use cached::wrap_cached;
pub use install::install;
