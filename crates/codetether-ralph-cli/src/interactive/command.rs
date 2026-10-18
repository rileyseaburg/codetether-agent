//! Ralph interactive subcommand dispatch and usage behavior.
use super::{Host, Registry, View, run_command, status};
use std::{path::Path, sync::Arc};
/// Dispatch the arguments following an interactive `/ralph` command.
/// # Arguments
/// * `view` - Client presentation adapter.
/// * `cwd` - Working directory for relative PRD paths.
/// * `model` - Session's current optional model choice.
/// * `registry` - Available provider registry.
/// * `rest` - Raw subcommand and arguments.
/// # Returns
/// False for a bare command so the caller can open the monitor; true otherwise.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_ralph_cli::interactive::Host, R: codetether_ralph_cli::interactive::Registry, V: codetether_ralph_cli::interactive::View>(view: &mut V) {
/// let handled = codetether_ralph_cli::interactive::handle::<H, R, V>(view, std::path::Path::new("."), None, None, "status");
/// # }
/// ```
pub fn handle<H: Host, R: Registry, V: View>(
    view: &mut V,
    cwd: &Path,
    model: Option<&str>,
    registry: Option<&Arc<R>>,
    rest: &str,
) -> bool {
    let rest = rest.trim();
    if rest.is_empty() {
        return false;
    }
    let mut parts = rest.split_whitespace();
    let verb = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();
    match verb {
        "run" => run_command::run::<H, R, V>(view, cwd, model, registry, &args),
        "status" => {
            status::show(view);
            true
        }
        _ => {
            view.set_status(
                "Usage: /ralph [run <prd.json> [--iters N]] | /ralph status".to_string(),
            );
            true
        }
    }
}
