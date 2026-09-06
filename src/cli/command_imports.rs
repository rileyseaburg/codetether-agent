// Argument types used by the top-level command enumeration.
use super::{
    A2aArgs, AuthArgs, BenchmarkArgs, CleanupArgs, ContextArgs, ForageArgs,
    GitCredentialHelperArgs, IndexArgs, McpArgs, ModelsArgs, MoltbookArgs, OkrArgs, OracleArgs,
    PrArgs, RalphArgs, RlmArgs, RunArgs, SearchArgs, ServeArgs, SpawnArgs, StatsArgs, SwarmArgs,
    SwarmSubagentArgs, TuiArgs, WorktreeArgs, approval, browserctl, clipboard,
    config_args::ConfigArgs, connect,
};
use clap::Subcommand;