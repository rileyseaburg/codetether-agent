//! Core TetherScript execution dispatch.

mod browser;
mod computer;
mod computer_invoke;
mod computer_lists;
mod computer_narrow;
mod computer_origin;
mod computer_payload;
mod computer_policy;
mod computer_scopes;
mod computer_value;
mod finish;
mod host;
mod host_grants;
mod interp;
mod outcome;
mod parse;
mod unwind;

use anyhow::Result;
use serde_json::Value;

pub use browser::BrowserGrant;
pub use computer::ComputerGrant;
pub use outcome::TetherScriptOutcome;

/// Run a TetherScript hook through the interpreter or capability host.
pub fn run(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
    browser: BrowserGrant,
    computer: ComputerGrant,
) -> Result<TetherScriptOutcome> {
    if browser.endpoint.is_some() || computer.enabled {
        host::run(source_name, source, hook, args, browser, computer)
    } else {
        interp::run(source_name, source, hook, args)
    }
}
