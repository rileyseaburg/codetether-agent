//! CodeTether Agent - A2A-native AI coding agent
//!
//! A Rust implementation of an AI coding agent with first-class support for the
//! A2A (Agent-to-Agent) protocol and the CodeTether ecosystem.
//!
//! By default, runs as an A2A worker connecting to the CodeTether platform.
//! Use the 'tui' subcommand for interactive terminal mode.
//!
//! SECURITY: All API keys and secrets are loaded exclusively from HashiCorp Vault.
//! Environment variables are NOT used for secrets (only for Vault connection bootstrap).

mod a2a;
mod agent;
mod cli;
mod config;
mod provider;
pub mod secrets;
mod server;
mod session;
mod tool;
mod tui;

use clap::Parser;
use cli::{A2aArgs, Cli, Command};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize HashiCorp Vault connection for secrets
    if let Ok(secrets_manager) = secrets::SecretsManager::from_env().await {
        if secrets_manager.is_connected() {
            tracing::info!("Connected to HashiCorp Vault for secrets management");
        }
        // Store in global
        let _ = secrets::init_from_manager(secrets_manager);
    } else {
        tracing::warn!("HashiCorp Vault not configured - no provider API keys will be available");
        tracing::warn!("Set VAULT_ADDR and VAULT_TOKEN environment variables to connect");
    }

    let cli = Cli::parse();

    match cli.command {
        Some(Command::Tui(args)) => {
            let project = args.project.or(cli.project);
            tui::run(project).await
        }
        Some(Command::Serve(args)) => server::serve(args).await,
        Some(Command::Run(args)) => cli::run::execute(args).await,
        Some(Command::Worker(args)) => a2a::worker::run(args).await,
        Some(Command::Config(args)) => cli::config::execute(args).await,
        None => {
            // Default: A2A worker mode
            // Check if we have a server URL from args or environment
            if let Some(server) = cli.server {
                let args = A2aArgs {
                    server,
                    token: cli.token,
                    name: cli.name,
                    codebases: None,
                    auto_approve: "safe".to_string(),
                    email: None,
                    push_url: None,
                };
                a2a::worker::run(args).await
            } else {
                // No server specified, show help
                eprintln!("CodeTether Agent v{}", env!("CARGO_PKG_VERSION"));
                eprintln!();
                eprintln!("Usage: codetether --server <URL> [OPTIONS]");
                eprintln!("   or: codetether tui                  # Interactive terminal mode");
                eprintln!("   or: codetether serve                # Start API server");
                eprintln!("   or: codetether run <message>        # Non-interactive mode");
                eprintln!();
                eprintln!("Environment variables:");
                eprintln!("  CODETETHER_SERVER   A2A server URL");
                eprintln!("  CODETETHER_TOKEN    Authentication token");
                eprintln!();
                eprintln!("HashiCorp Vault (required for API keys):");
                eprintln!("  VAULT_ADDR          Vault server address");
                eprintln!("  VAULT_TOKEN         Vault authentication token");
                eprintln!("  VAULT_MOUNT         KV secrets mount (default: secret)");
                eprintln!("  VAULT_SECRETS_PATH  Path prefix (default: codetether/providers)");
                eprintln!();
                eprintln!("Run 'codetether --help' for full options.");
                std::process::exit(1);
            }
        }
    }
}
