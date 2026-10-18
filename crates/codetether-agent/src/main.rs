//! CodeTether Agent - A2A-native AI coding agent
//!
//! A Rust implementation of an AI coding agent with first-class support for the
//! A2A (Agent-to-Agent) protocol and the CodeTether ecosystem.
//!
//! Running `codetether` without a subcommand opens the interactive TUI.
//! Use the `worker` subcommand to connect an A2A worker to CodeTether.
//!
//! SECURITY: Provider credentials are loaded from HashiCorp Vault first.
//! Local development may also use env/AWS fallback credentials unless
//! `CODETETHER_DISABLE_ENV_FALLBACK=1` is set for Vault-only mode.

#![recursion_limit = "512"]

use cli::{Cli, Command};
use codetether_agent::{
    a2a, benchmark, cli, forage, github_pr, indexer, mcp, mux, provider, ralph, server, swarm,
    tool, tui,
};

mod application;
use codetether_cleanup_cli as cleanup_cli;
use codetether_native_crash::startup as startup_crash;
mod worker_cli;
use codetether_worktree_cli as worktree_cli;

mod mcp_cli;
mod ralph_cli;

#[tokio::main(worker_threads = 8)]
async fn main() -> anyhow::Result<()> {
    codetether_cli_startup::lifecycle::run::<application::Native>().await
}

include!("command_dispatch.rs");
