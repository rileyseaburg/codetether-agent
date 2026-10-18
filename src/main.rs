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

use cli::{Cli, Command};
use codetether_agent::{
    a2a, benchmark, bus, cli, forage, github_pr, indexer, mcp, mux, provider, ralph, server, swarm,
    tool, tui,
};

mod application;
use codetether_cleanup_cli as cleanup_cli;
mod startup_crash;
mod worker_cli;
use codetether_worktree_cli as worktree_cli;

mod mcp_cli;
mod ralph_cli;

#[tokio::main(worker_threads = 8)]
async fn main() -> anyhow::Result<()> {
    codetether_cli_startup::lifecycle::run::<application::Native>().await
}

codetether_cli_startup::define_command_dispatch!(
    dispatch, Cli, Command;
    bonsai = cli::bonsai::run,
    windows = cli::windows::run,
    tui = codetether_tui_cli::execute::<tui::app::run::Cli>,
    mux = mux::execute,
    serve = server::serve,
    run = cli::run::execute,
    pr = github_pr::run,
    approval = cli::approval::execute,
    clipboard = cli::clipboard::execute,
    models = cli::execute_models,
    index = indexer::run,
    auth = cli::auth::execute,
    vault = cli::vault::execute,
    connect = cli::connect::execute,
    forage = forage::execute,
    search = cli::search::execute,
    browser = cli::browserctl::execute,
    context = cli::context::execute,
    worker = worker_cli::execute,
    credential = a2a::git_credentials::run_git_credential_helper,
    spawn = a2a::spawn::run,
    config = cli::config::execute,
    swarm = codetether_swarm_cli::execute::<swarm::SwarmExecutor>,
    subagent = swarm::remote_subtask::run_swarm_subagent,
    rlm = codetether_rlm_cli::execute::<provider::ProviderRegistry>,
    oracle = cli::oracle::execute,
    ralph = codetether_ralph_cli::execute::<ralph_cli::NativeHost>,
    mcp = codetether_mcp_cli::dispatch::<mcp_cli::NativeHost, mcp_cli::ServerHost>,
    stats = codetether_telemetry_cli::execute,
    benchmark = codetether_benchmark_cli::execute::<benchmark::BenchmarkRunner>,
    cleanup = cleanup_cli::run,
    worktree = worktree_cli::run,
    moltbook = codetether_moltbook_cli::execute,
    okr = codetether_okr_cli::execute
);
