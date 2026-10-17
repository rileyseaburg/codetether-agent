//! GitHub CLI auth payloads for bash tool execution.
//!
//! This module stores the environment variables and redaction list that the
//! bash tool injects when it needs GitHub App credentials for `gh` commands.
//!
//! # Examples
//!
//! ```ignore
//! let auth = GitHubCommandAuth::new("secret".into(), None);
//! assert!(!auth.redactions.is_empty());
//! ```

/// Shell environment for authenticated GitHub CLI commands.
///
/// The bash tool injects these variables when a command needs access to a
/// CodeTether-provisioned GitHub token. The same token is also tracked for
/// output redaction.
///
/// # Examples
///
/// ```ignore
/// let auth = GitHubCommandAuth::new("secret".into(), Some("/tmp/gh".into()));
/// assert_eq!(auth.env[0].0, "GH_TOKEN");
/// ```
pub struct GitHubCommandAuth {
    /// Environment variables applied before invoking the shell command.
    pub env: Vec<(&'static str, String)>,
    /// Secrets that should be redacted from command output.
    pub redactions: Vec<String>,
}

impl GitHubCommandAuth {
    /// Builds an auth payload from a GitHub token and optional config directory.
    ///
    /// The constructor duplicates the token into the environment expected by
    /// the `gh` CLI and also registers the token for output redaction.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let auth = GitHubCommandAuth::new("secret".into(), None);
    /// assert_eq!(auth.redactions.len(), 1);
    /// ```
    pub(super) fn new(password: String, config_dir: Option<String>) -> Self {
        let mut env = vec![
            ("GH_TOKEN", password.clone()),
            ("GITHUB_TOKEN", password.clone()),
            ("GH_HOST", "github.com".to_string()),
            ("GH_PROMPT_DISABLED", "1".to_string()),
        ];
        if let Some(config_dir) = config_dir {
            env.push(("GH_CONFIG_DIR", config_dir));
        }
        Self {
            env,
            redactions: vec![password],
        }
    }
}
