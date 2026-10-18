//! Webview header and status bar presentation.
//!
//! Supply borrowed display data through [header::Header] and [prefix::Status].
//! [header::render] draws the header; [prefix::build] prepares badges for
//! [status::render], which combines them with an existing token summary.
pub mod header;
pub mod prefix;
pub mod status;
