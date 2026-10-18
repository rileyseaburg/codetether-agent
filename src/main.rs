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
    tool,
};

mod application;
mod cleanup_cli;
mod startup_crash;
mod tui_cli;
mod worker_cli;
mod worktree_cli;

mod benchmark_cli;
mod mcp_cli;
mod mcp_serve;
mod native_cli;
mod ralph_cli;
mod rlm_cli;
mod swarm_cli;

#[tokio::main(worker_threads = 8)]
async fn main() -> anyhow::Result<()> {
    codetether_cli_startup::lifecycle::run::<application::Native>().await
}

async fn dispatch(cli: Cli) -> anyhow::Result<()> {
    let mcp_control_plane_url = cli.server.clone();
    let mcp_control_plane_token = cli.token.clone();
    let (mcp_project_root, mcp_project_was_explicit) = (cli.project.clone(), cli.project.is_some());

    match cli.command {
        Some(Command::Bonsai(args)) => cli::bonsai::run(args).await,
        Some(Command::Windows(args)) => cli::windows::run(args).await,
        Some(Command::Tui(args)) => tui_cli::execute(Some(args), cli.project, cli.yolo).await,
        Some(Command::Mux(args)) => mux::execute(args).await,
        Some(Command::Serve(args)) => server::serve(args).await,
        Some(Command::Run(args)) => cli::run::execute(args).await,
        Some(Command::Pr(args)) => github_pr::run(args).await,
        Some(Command::Approval(args)) => cli::approval::execute(args),
        Some(Command::Clipboard(args)) => cli::clipboard::execute(args),
        Some(Command::Models(args)) => cli::execute_models(args).await,
        Some(Command::Index(args)) => indexer::run(args).await,
        Some(Command::Auth(args)) => cli::auth::execute(args).await,
        Some(Command::Vault(args)) => cli::vault::execute(args).await,
        Some(Command::Connect(args)) => cli::connect::execute(args).await,
        Some(Command::Forage(args)) => forage::execute(args).await,
        Some(Command::Search(args)) => cli::search::execute(args).await,
        Some(Command::Browserctl(args)) => cli::browserctl::execute(args).await,
        Some(Command::Context(args)) => cli::context::execute(args).await,
        Some(Command::Worker(args)) => worker_cli::execute(args).await,
        Some(Command::GitCredentialHelper(args)) => {
            a2a::git_credentials::run_git_credential_helper(&args).await
        }
        Some(Command::Spawn(args)) => a2a::spawn::run(args).await,
        Some(Command::Config(args)) => cli::config::execute(args).await,
        Some(Command::Swarm(args)) => swarm_cli::execute(args).await,
        Some(Command::SwarmSubagent(args)) => swarm::remote_subtask::run_swarm_subagent(args).await,
        Some(Command::Rlm(args)) => rlm_cli::run(args).await,
        Some(Command::Oracle(args)) => cli::oracle::execute(args).await,
        Some(Command::Ralph(args)) => ralph_cli::execute(args).await,
        Some(Command::Mcp(args)) => {
            // Resolve the target MCP server command from either `--command "cmd args"`
            // (flag form) or trailing positional args (e.g. `connect npx -y @x/foo /path`).
            let resolved_command = codetether_mcp_cli::resolve_command(&args);
            match args.action.as_str() {
                "serve" => {
                    mcp_serve::serve(
                        &args,
                        &mcp_control_plane_url,
                        &mcp_control_plane_token,
                        &mcp_project_root,
                        mcp_project_was_explicit,
                    )
                    .await
                }
                _ => mcp_cli::execute(args, resolved_command).await,
            }
        }
        Some(Command::Stats(args)) => codetether_telemetry_cli::execute(args),
        Some(Command::Benchmark(args)) => benchmark_cli::execute(args).await,
        Some(Command::Cleanup(args)) => cleanup_cli::run(args).await,
        Some(Command::Worktree(args)) => worktree_cli::run(args).await,
        Some(Command::Moltbook(args)) => codetether_moltbook_cli::execute(args).await,

        // OKR commands
        Some(Command::Okr(args)) => codetether_okr_cli::execute(args).await,
        None => tui_cli::execute(None, cli.project, cli.yolo).await,
    }
}
