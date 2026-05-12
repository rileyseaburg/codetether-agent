use anyhow::Result;
use serde_json::Value;
use tetherscript::output;

use crate::tool::tetherscript::convert::json_to_tetherscript;

use super::outcome::TetherScriptOutcome;

const OUTPUT_LIMIT: usize = 64 * 1024;

/// Run via raw Interpreter without capability grants.
pub fn run(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
) -> Result<TetherScriptOutcome> {
    let interp = super::parse::interpreter(&source_name, &source)?;
    let callee = interp
        .globals
        .borrow()
        .get(&hook)
        .map_err(|e| anyhow::anyhow!("{source_name}: hook '{hook}': {e}"))?;
    let ts_args: Vec<_> = args.into_iter().map(json_to_tetherscript).collect();
    let (stdout, call_result) =
        output::with_capture(OUTPUT_LIMIT, || interp.call(&callee, &ts_args));
    super::finish::finish(stdout, call_result)
}
