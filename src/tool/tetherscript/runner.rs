//! Bind shared TetherScript execution to the application's computer backend.
mod computer;
pub use codetether_tetherscript_runtime::{BrowserGrant, ComputerGrant, TetherScriptOutcome};
/// Run a hook using the application's existing computer-use policy and backend.
///
/// # Arguments
/// * `source_name` - Label used in parse and load errors.
/// * `source` - Plugin source.
/// * `hook` - Function to invoke.
/// * `args` - JSON arguments converted to script values.
/// * `browser` - Browser capability settings.
/// * `computer` - Computer capability settings.
/// * `progress_id` - Optional invocation identifier for live output.
/// # Returns
/// Captured text, return value and success state.
/// # Errors
/// Returns parse, load or capability-host execution errors.
pub fn run(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<serde_json::Value>,
    browser: BrowserGrant,
    computer: ComputerGrant,
    progress_id: Option<String>,
) -> anyhow::Result<TetherScriptOutcome> {
    codetether_tetherscript_runtime::run::<computer::ApplicationHost>(
        source_name,
        source,
        hook,
        args,
        browser,
        computer,
        progress_id,
    )
}
#[cfg(test)]
#[path = "runner/process_arity_tests.rs"]
mod process_arity_tests;
#[cfg(test)]
#[path = "runner/process_migration_tests.rs"]
mod process_migration_tests;
