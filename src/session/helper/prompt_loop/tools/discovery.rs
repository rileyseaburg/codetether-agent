//! Runtime discovery and activation of compact-profile tools.

use super::super::Runner;
use super::call::Call;
use anyhow::Result;
use codetether_tool_catalog::promote;

/// Handles the virtual `list_tools` call.
///
/// A request for one exact tool both returns its schema and adds that schema to
/// the next provider request. This keeps the default catalog compact without
/// making the rest of the executable registry unreachable.
pub(super) async fn run(runner: &mut Runner<'_>, call: &Call) -> Result<bool> {
    promote(
        &runner.model.tools,
        &mut runner.model.advertised,
        &call.input,
    );
    let content = super::super::super::bootstrap::list_tools_bootstrap_output(
        &runner.model.tools,
        &call.input,
    );
    super::simple::record(runner, call, content, true).await;
    Ok(false)
}
