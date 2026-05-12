use anyhow::Result;

use crate::tool::tetherscript::runner::TetherScriptOutcome;

/// Outcome of a TetherScript plugin run.
pub enum TetherScriptRunResult {
    Finished(Result<TetherScriptOutcome>),
    Timeout(u64),
}
