//! Ralph loop result output with lazily constructed text status.
use codetether_ralph_core::RalphState;
/// Print JSON state or the caller-supplied status text.
/// # Arguments
/// * `result` - Completed loop state.
/// * `json` - Whether to serialize the state as JSON.
/// * `markdown` - Status rendering, invoked only for text output.
/// # Returns
/// Unit after output.
/// # Errors
/// Returns JSON serialization errors.
/// # Examples
/// ```no_run
/// # fn example(result: &codetether_ralph_core::RalphState) {
/// let _ = codetether_ralph_cli::print_result(result, true, String::new);
/// # }
/// ```
pub fn print_result(
    result: &RalphState,
    json: bool,
    markdown: impl FnOnce() -> String,
) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", markdown());
    }
    Ok(())
}
