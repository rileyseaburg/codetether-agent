//! CLI command definitions and handlers

#[cfg(test)]
mod access_mode_args_tests;
pub use codetether_approval_cli as approval;
pub mod auth;
pub use codetether_bedrock_auth_cli as auth_bedrock;
include!("platform_commands.rs");
pub use codetether_cleanup_cli as cleanup_args;
pub mod command;
pub use codetether_config_cli as config;
use codetether_config_cli as config_args;
pub use codetether_connect_cli as connect;
mod models_args;
pub use codetether_oracle_cli as oracle;
mod root_args;
use codetether_cli_options as run_args;
pub use codetether_run_resume as run_checkpoint;
pub mod search;
use codetether_cli_options as swarm_subagent_args;
use codetether_cli_options as tui_args;
pub use codetether_search_cli as search_render;
pub use codetether_worktree_cli as worktree_args;

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

#[cfg(test)]
mod run_config {
    pub use codetether_run_policy::effective_access_mode;
}
