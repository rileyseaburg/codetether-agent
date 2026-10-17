//! TetherScript hook dispatch entry point.
//!
//! Routes a hook to the capability host (when browser/computer grants are
//! present) or the plain interpreter otherwise. Kept separate from the module
//! table in [`super`] to honor the 50-line file budget.

use crate::Host;
use anyhow::Result;
use serde_json::Value;

use super::{BrowserGrant, ComputerGrant, TetherScriptOutcome, host, interp};

/// Run a TetherScript hook through the interpreter or capability host.
///
/// # Arguments
/// * `source_name` - Label attached to parse and load errors.
/// * `source` - Plugin source to execute.
/// * `hook` - Function name to call.
/// * `args` - JSON arguments converted to script values.
/// * `browser` - Optional browser access and restrictions.
/// * `computer` - Computer access and restrictions.
/// * `progress_id` - Invocation identifier for live process output.
/// # Returns
/// Captured output, JSON return value and success status.
/// # Errors
/// Returns parse, load, hook lookup or capability-host call errors.
/// # Examples
/// ```rust,no_run
/// # fn example<H:codetether_tetherscript_runtime::Host>()->anyhow::Result<()> {
/// use codetether_tetherscript_runtime::{BrowserGrant,ComputerGrant,run};
/// let result=run::<H>("example".into(),"fn main() { return 1 }".into(),"main".into(),vec![],
///     BrowserGrant{endpoint:None,origins:vec![],scopes:vec![]},
///     ComputerGrant{enabled:false,origins:vec![],scopes:vec![]},None)?;
/// assert!(result.success);
/// # Ok(()) }
/// ```
pub fn run<H: Host>(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
    browser: BrowserGrant,
    computer: ComputerGrant,
    progress_id: Option<String>,
) -> Result<TetherScriptOutcome> {
    if browser.endpoint.is_some() || computer.enabled {
        return host::run::<H>(
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
