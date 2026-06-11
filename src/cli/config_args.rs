use clap::{Parser, Subcommand};

/// Arguments for `codetether config`.
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
#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// Manage trust for project-local configuration
    Project(ProjectArgs),
}

/// Arguments for `codetether config project`.
#[derive(Parser, Debug)]
pub struct ProjectArgs {
    /// Project trust operation to execute.
    #[command(subcommand)]
    pub command: ProjectCommand,
}

/// Trust operations for project-local configuration.
#[derive(Subcommand, Debug, Clone, Copy)]
pub enum ProjectCommand {
    /// Show trust status for the current project
    Status,
    /// Trust project-local configuration for the current project
    Trust,
    /// Remove trust for the current project
    Untrust,
}
