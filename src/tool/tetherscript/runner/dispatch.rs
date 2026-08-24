//! TetherScript hook dispatch entry point.
//!
//! Routes a hook to the capability host (when browser/computer grants are
//! present) or the plain interpreter otherwise. Kept separate from the module
//! table in [`super`] to honor the 50-line file budget.

use anyhow::Result;

use super::{BrowserGrant, ComputerGrant, TetherScriptOutcome, host, interp};

/// Run a TetherScript hook through the interpreter or capability host.
pub fn run(
    request: crate::tool::tetherscript::task::TetherScriptRun,
) -> Result<TetherScriptOutcome> {
    let browser = BrowserGrant {
        endpoint: request.grant_browser,
        origins: request.browser_origin,
        scopes: request.browser_scope,
    };
    let computer = ComputerGrant {
        enabled: request.grant_computer,
        origins: request.computer_origin,
        scopes: request.computer_scope,
    };
    if browser.endpoint.is_some() || computer.enabled {
        return host::run(
            request.source_name,
            request.source,
            request.hook,
            request.args,
            browser,
            computer,
            request.progress_id,
            request.process,
        );
    }
    interp::run(
        request.source_name,
        request.source,
        request.hook,
        request.args,
        request.progress_id,
        request.process,
    )
}
