//! task run_result support for the TetherScript plugin tool.
use anyhow::Result;

use codetether_tetherscript_runtime::TetherScriptOutcome;

/// Outcome of a TetherScript plugin run.
pub enum TetherScriptRunResult {
    Finished(Result<TetherScriptOutcome>),
    Timeout(u64),
}
