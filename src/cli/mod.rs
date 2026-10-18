//! CLI command definitions and handlers

#[cfg(test)]
mod access_mode_args_tests;
pub mod approval;
pub mod auth;
pub mod auth_bedrock;
include!("platform_commands.rs");
pub mod cleanup_args;
pub mod command;
pub mod config;
mod config_args;
pub mod connect;
pub mod context;
pub mod go_ralph;
mod models_args;
pub mod oracle;
mod root_args;
pub mod run;
mod run_args;
pub mod run_checkpoint;
mod run_config;
pub mod run_loop;
pub mod search;
pub mod search_render;
mod swarm_subagent_args;
mod tui_args;
pub mod worktree_args;

pub use codetether_context_cli::{
    ContextArgs, ContextBrowseArgs, ContextBrowseCommand, ContextCommand, ContextResetArgs,
    ContextShowTurnArgs,
};
pub use codetether_cookie_auth_cli::CookieAuthArgs;
pub use codetether_indexer_cli::IndexArgs;
pub use codetether_search_cli::SearchArgs;
pub use codetether_user_auth_cli::LoginAuthArgs;
pub use codetether_user_auth_cli::RegisterAuthArgs;
pub use command::Command;
pub use config_args::{ConfigArgs, ConfigCommand, ProjectArgs, ProjectCommand};
pub use worktree_args::{
    WorktreeArgs, WorktreeCommand, WorktreeListArgs, WorktreeOpenArgs, WorktreeWorkspaceArgs,
};
pub use {
    cleanup_args::CleanupArgs, models_args::ModelsArgs, run_args::RunArgs,
    swarm_subagent_args::SwarmSubagentArgs, tui_args::TuiArgs,
};
pub use {models_args::execute as execute_models, root_args::Cli};

pub use codetether_auth_cli::{AuthArgs, AuthCommand, CodexAuthArgs, CopilotAuthArgs};

pub use codetether_oracle_cli::{OracleArgs, OracleCommand, OracleSyncArgs, OracleValidateArgs};

pub use codetether_cli_options::{
    A2aArgs, BenchmarkArgs, CreatePrArgs, ForageArgs, GitCredentialHelperArgs, McpArgs,
    MoltbookArgs, MoltbookCommand, MoltbookCommentArgs, MoltbookPostArgs, MoltbookRegisterArgs,
    MoltbookSearchArgs, MoltbookUpdateProfileArgs, OkrArgs, PrArgs, PrCommand, RalphArgs, RlmArgs,
    ServeArgs, SpawnArgs, StatsArgs, SwarmArgs, WorkerServerArgs,
};
