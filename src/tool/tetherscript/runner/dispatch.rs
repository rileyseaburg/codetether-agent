//! TetherScript hook dispatch entry point.
//!
//! Routes a hook to the capability host (when browser/computer grants are
//! present) or the plain interpreter otherwise. Kept separate from the module
//! table in [`super`] to honor the 50-line file budget.

use anyhow::Result;
use serde_json::Value;

use super::{BrowserGrant, ComputerGrant, TetherScriptOutcome, host, interp};

/// Run a TetherScript hook through the interpreter or capability host.
pub fn run(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
    browser: BrowserGrant,
    computer: ComputerGrant,
    progress_id: Option<String>,
) -> Result<TetherScriptOutcome> {
    if browser.endpoint.is_some() || computer.enabled {
        return host::run(
            source_name,
            source,
            hook,
            args,
            browser,
            computer,
            progress_id,
        );
    }
    interp::run(source_name, source, hook, args, progress_id)
}
