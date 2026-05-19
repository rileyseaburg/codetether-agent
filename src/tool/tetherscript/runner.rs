//! Core TetherScript execution dispatch.

mod browser;
mod finish;
mod host;
mod interp;
mod outcome;
mod parse;
mod unwind;

use anyhow::Result;
use serde_json::Value;

pub use browser::BrowserGrant;
pub use outcome::TetherScriptOutcome;

/// Run a TetherScript hook through the interpreter or capability host.
pub fn run(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
    browser: BrowserGrant,
) -> Result<TetherScriptOutcome> {
    if browser.endpoint.is_some() {
        host::run(source_name, source, hook, args, browser)
    } else {
        interp::run(source_name, source, hook, args)
    }
}
