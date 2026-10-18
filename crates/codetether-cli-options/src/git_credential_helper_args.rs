//! CLI syntax for GitCredentialHelperArgs.
use clap::Parser;
/// Parsed options for the git credential helper args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::GitCredentialHelperArgs::try_parse_from(["helper","--workspace-id","workspace"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug, Clone)]
pub struct GitCredentialHelperArgs {
    /// Workspace/codebase ID for which Git credentials should be minted.
    #[arg(long = "workspace-id", alias = "codebase-id")]
    pub workspace_id: String,

    /// CodeTether server URL.
    #[arg(long, env = "CODETETHER_SERVER")]
    pub server: Option<String>,

    /// CodeTether bearer token.
    #[arg(long, env = "CODETETHER_TOKEN")]
    pub token: Option<String>,

    /// Worker ID requesting the credential.
    #[arg(long, env = "CODETETHER_WORKER_ID")]
    pub worker_id: Option<String>,

    /// Git helper operation (get/store/erase). Git passes this positionally.
    pub operation: Option<String>,
}
