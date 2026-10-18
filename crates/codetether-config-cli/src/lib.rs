//! Configuration and project trust command execution.
//!
//! [ConfigArgs] selects inspection, initialization, key updates or project
//! trust management. [execute] delegates storage to the shared config crate
//! and prints the current command output.

mod set;
mod status;
mod trust;

mod args;
use anyhow::Result;
pub use args::{ConfigArgs, ConfigCommand, ProjectArgs, ProjectCommand};
use codetether_config::Config;

/// Execute a parsed configuration command.
///
/// # Arguments
/// * `args` - Inspection, initialization, setting or project-trust operation.
/// # Returns
/// Unit after completing the operation and printing its result.
/// # Errors
/// Returns configuration, serialization, path or trust-record errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_config_cli::execute(codetether_config_cli::ConfigArgs {
///     command: None, show: true, init: false, set: None,
/// }).await
/// # }
/// ```
pub async fn execute(args: ConfigArgs) -> Result<()> {
    if let Some(command) = args.command {
        return match command {
            ConfigCommand::Project(args) => trust::execute(args.command),
        };
    }

    if args.show {
        let config = Config::load().await?;
        println!("{}", toml::to_string_pretty(&config)?);
        status::print_effective(&config)?;
        return Ok(());
    }

    if args.init {
        Config::init_default().await?;
        println!("Configuration initialized");
        return Ok(());
    }

    if let Some(kv) = args.set {
        return set::execute(&kv).await;
    }

    // Default: show help
    println!("Use --show, --init, --set key=value, or config project status");
    Ok(())
}
