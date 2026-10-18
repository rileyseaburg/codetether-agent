//! Argument contracts for config commands and project-trust operations.

use clap::{Parser, Subcommand};

/// Arguments for `codetether config`.
///
/// # Examples
/// ```
/// let args = codetether_config_cli::ConfigArgs { command: None, show: true, init: false, set: None };
/// assert!(args.show);
/// ```
#[derive(Parser, Debug)]
pub struct ConfigArgs {
    /// Optional focused configuration subcommand.
    #[command(subcommand)]
    pub command: Option<ConfigCommand>,

    /// Show current configuration
    #[arg(long)]
    pub show: bool,

    /// Initialize default configuration
    #[arg(long)]
    pub init: bool,

    /// Set a configuration value
    #[arg(long)]
    pub set: Option<String>,
}

/// Focused `codetether config` subcommands.
///
/// # Examples
/// ```
/// let command = codetether_config_cli::ConfigCommand::Project(codetether_config_cli::ProjectArgs { command: codetether_config_cli::ProjectCommand::Status });
/// assert!(matches!(command, codetether_config_cli::ConfigCommand::Project(_)));
/// ```
#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// Manage trust for project-local configuration
    Project(ProjectArgs),
}

/// Arguments for `codetether config project`.
///
/// # Examples
/// ```
/// let args = codetether_config_cli::ProjectArgs { command: codetether_config_cli::ProjectCommand::Status };
/// assert!(matches!(args.command, codetether_config_cli::ProjectCommand::Status));
/// ```
#[derive(Parser, Debug)]
pub struct ProjectArgs {
    /// Project trust operation to execute.
    #[command(subcommand)]
    pub command: ProjectCommand,
}

/// Trust operations for project-local configuration.
///
/// # Examples
/// ```
/// let command = codetether_config_cli::ProjectCommand::Status;
/// match command {
///     codetether_config_cli::ProjectCommand::Status => {},
///     codetether_config_cli::ProjectCommand::Trust | codetether_config_cli::ProjectCommand::Untrust => {},
/// }
/// ```
#[derive(Subcommand, Debug, Clone, Copy)]
pub enum ProjectCommand {
    /// Show trust status for the current project
    Status,
    /// Trust project-local configuration for the current project
    Trust,
    /// Remove trust for the current project
    Untrust,
}
