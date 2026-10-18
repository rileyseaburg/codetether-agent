//! Access mode precedence for CLI runs.
use codetether_cli_options::RunArgs;
use codetether_config::AccessMode;

/// Return the effective access mode, letting `--yolo` act as `--access-mode full`.
/// Return the CLI access mode, with yolo taking precedence.
/// # Arguments
/// * `args` - CLI access mode and yolo options.
/// # Returns
/// Full access for yolo; otherwise the explicit optional access mode.
/// # Examples
/// ```no_run
/// # fn example(args: &codetether_cli_options::RunArgs) {
/// let mode = codetether_run_policy::effective_access_mode(args);
/// # }
/// ```
pub fn effective_access_mode(args: &RunArgs) -> Option<AccessMode> {
    if args.yolo {
        Some(AccessMode::Full)
    } else {
        args.access_mode
    }
}
