//! Dispatch explicit or default TUI launch settings.
use crate::Host;
use codetether_a2a_http::peer_startup::SpawnOptions;
use codetether_cli_options::TuiArgs;
use std::path::PathBuf;
/// Launch the TUI using explicit arguments or the CLI default behavior.
/// # Arguments
/// `args` is present for the TUI subcommand; `project` and `yolo` are root CLI settings.
/// # Returns
/// Completes when the native terminal runtime returns.
/// # Errors
/// Returns native TUI startup or execution errors.
/// # Examples
/// ```no_run
/// async fn launch<H: codetether_tui_cli::Host>() {
///     codetether_tui_cli::execute::<H>(None, None, false).await.unwrap();
/// }
/// ```
pub async fn execute<H: Host>(
    args: Option<TuiArgs>,
    project: Option<PathBuf>,
    yolo: bool,
) -> anyhow::Result<()> {
    match args {
        Some(args) => crate::explicit::execute::<H>(args, project, yolo).await,
        None => H::run(project, false, Some(SpawnOptions::auto()), None, yolo, None).await,
    }
}
