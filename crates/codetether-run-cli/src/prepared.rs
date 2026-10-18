//! Prepared run inputs shared by route selection.
use codetether_cli_options::RunArgs;
use codetether_config::Config;
use std::path::PathBuf;
pub(super) struct Prepared {
    pub args: RunArgs,
    pub message: String,
    pub jsonl_output: bool,
    pub config: Config,
    pub workspace_dir: PathBuf,
    pub knowledge_snapshot: Option<PathBuf>,
}
