// Argument owners used to assemble the top-level CLI schema.
use clap::Subcommand;
use codetether_approval_cli as approval;
use codetether_auth_cli::AuthArgs;
use codetether_browser_cli as browserctl;
use codetether_cleanup_cli::CleanupArgs;
use codetether_cli_options::*;
use codetether_clipboard_cli as clipboard;
use codetether_config_cli::ConfigArgs;
use codetether_connect_cli as connect;
use codetether_context_cli::ContextArgs;
use codetether_indexer_cli::IndexArgs;
use codetether_models_cli::ModelsArgs;
use codetether_mux_cli::MuxArgs;
use codetether_oracle_cli::OracleArgs;
use codetether_search_cli::SearchArgs;
use codetether_worktree_cli::WorktreeArgs;
