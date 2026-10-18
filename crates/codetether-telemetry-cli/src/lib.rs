//! Telemetry statistics command presentation and filtering.
//!
//! execute snapshots existing telemetry and renders the same text or JSON
//! report. Storage and process-wide counters remain owned by telemetry.
mod filtered;
mod json;
mod recent_files;
mod recent_tools;
mod summary;
mod tokens;
use codetether_cli_options::StatsArgs;
use codetether_telemetry::{TOKEN_USAGE, get_persistent_stats};
/// Render telemetry according to the existing command flags.
/// # Arguments
/// * `args` - Output mode, filters and record limits.
/// # Returns
/// Unit after writing the requested report to stdout.
/// # Errors
/// Returns JSON serialization failures.
/// # Examples
/// ```no_run
/// # fn example(args: codetether_cli_options::StatsArgs) {
/// let _ = codetether_telemetry_cli::execute(args);
/// # }
/// ```
pub fn execute(args: StatsArgs) -> anyhow::Result<()> {
    let persistent = get_persistent_stats();
    let token_snapshot = TOKEN_USAGE.global_snapshot();
    if args.json {
        json::print(&args, &persistent)?;
    } else {
        summary::print(&persistent, &token_snapshot);
        recent_tools::print(&args, &persistent);
        recent_files::print(&args, &persistent);
        tokens::print(&args);
        filtered::print(&args, &persistent);
    }
    Ok(())
}
