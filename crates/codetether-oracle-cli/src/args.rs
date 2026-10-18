//! Oracle validation and synchronization arguments.
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Oracle command options.
///
/// # Examples
/// ```
/// let args = codetether_oracle_cli::OracleArgs { command: codetether_oracle_cli::OracleCommand::Sync(codetether_oracle_cli::OracleSyncArgs { json: true }) };
/// ```
#[derive(Parser, Debug)]
pub struct OracleArgs {
    /// Oracle operation to execute.
    #[command(subcommand)]
    pub command: OracleCommand,
}

/// Select validation or synchronization.
///
/// # Examples
/// ```
/// let command = codetether_oracle_cli::OracleCommand::Sync(codetether_oracle_cli::OracleSyncArgs { json: true });
/// assert!(matches!(command, codetether_oracle_cli::OracleCommand::Sync(_)));
/// ```
#[derive(Subcommand, Debug)]
pub enum OracleCommand {
    /// Validate a FINAL(JSON) payload deterministically against source content
    Validate(OracleValidateArgs),
    /// Sync pending oracle spool files to MinIO/S3
    Sync(OracleSyncArgs),
}

/// Oracle command options.
///
/// # Examples
/// ```
/// let args = codetether_oracle_cli::OracleSyncArgs { json: true };
/// ```
#[derive(Parser, Debug)]
pub struct OracleSyncArgs {
    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

/// Oracle command options.
///
/// # Examples
/// ```
/// let args = codetether_oracle_cli::OracleValidateArgs { query: "inspect".into(), file: None, content: Some("source".into()), payload: Some("{}".into()), payload_file: None, json: true, persist: false };
/// ```
#[derive(Parser, Debug)]
pub struct OracleValidateArgs {
    /// Query associated with the payload (used as trace prompt)
    #[arg(long)]
    pub query: String,

    /// Source file to validate against
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// Source content to validate against (use - for stdin)
    #[arg(long)]
    pub content: Option<String>,

    /// FINAL(JSON) payload string
    #[arg(long)]
    pub payload: Option<String>,

    /// Path to a file containing FINAL(JSON) payload
    #[arg(long)]
    pub payload_file: Option<PathBuf>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Persist validated record to oracle storage pipeline
    #[arg(long)]
    pub persist: bool,
}
