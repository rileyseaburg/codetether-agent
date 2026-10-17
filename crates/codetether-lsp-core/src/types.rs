//! LSP protocol data and server configuration.
//!
//! Public types are re-exported here; focused modules own wire data, configuration,
//! and installation. Use `LspConfig` to configure a client connection.

use super::tetherscript;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{info, warn};
mod actions;
mod catalog;
mod completion;
mod config;
#[path = "detect_language.rs"]
mod detect_language;
mod diagnostic;
mod document;
mod geometry;
mod initialize;
mod install;
mod install_command;
mod linter_catalog;
mod linter_extensions;
mod notification;
mod overrides;
mod reference;
mod request;
mod response;
mod run_install;
mod rust_server;
mod symbols;
mod verify_install;
pub use actions::*;
pub use catalog::*;
pub use completion::*;
pub use config::*;
pub use detect_language::detect_language_from_path;
pub use diagnostic::*;
pub use document::*;
pub use geometry::*;
pub use initialize::*;
pub use install::*;
use install_command::*;
pub use linter_catalog::*;
pub use linter_extensions::*;
pub use notification::*;
pub use reference::*;
pub use request::*;
pub use response::*;
use run_install::run_install;
use rust_server::*;
pub use symbols::*;
use verify_install::verify_install;
