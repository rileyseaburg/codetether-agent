//! Language-server checks for proposed edits before user approval.
//!
//! [inspect] reconstructs proposed contents and uses the host's shared manager.
//! [scan::run] applies one interactive deadline across those contents while
//! diagnostic tasks retain ownership of initializing servers in the background.
//! [cooldown::reason] reports pending checks and temporary server failure backoff.
mod background;
pub mod budget;
pub mod check;
pub mod cooldown;
mod diagnostics;
mod files;
mod inspect;
mod report;
pub mod scan;
mod scan_file;
pub use inspect::inspect;
