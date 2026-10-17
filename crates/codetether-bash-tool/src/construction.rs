//! Construction defaults for the bash tool.
use super::BashTool;
use std::path::PathBuf;
impl BashTool {
    /// Create a tool with environment-derived sandboxing and a 120-second timeout.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool with these execution defaults.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_bash_tool::BashTool::new();
    /// ```
    pub fn new() -> Self {
        Self {
            timeout_secs: 120,
            sandboxed: codetether_bash_policy::mode::default_enabled(),
            default_cwd: None,
        }
    }

    /// Create a tool whose commands default to this working directory.
    ///
    /// # Arguments
    /// `default_cwd` is the working directory used when a call omits `cwd`.
    /// # Returns
    /// A tool with these execution defaults.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_bash_tool::BashTool::with_cwd("/workspace".into());
    /// ```
    pub fn with_cwd(default_cwd: PathBuf) -> Self {
        Self {
            default_cwd: Some(default_cwd),
            ..Self::new()
        }
    }

    /// Create a tool with a custom timeout in seconds.
    ///
    /// # Arguments
    /// `timeout_secs` is the default time allowed for each command.
    /// # Returns
    /// A tool with these execution defaults.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_bash_tool::BashTool::with_timeout(30);
    /// ```
    pub fn with_timeout(timeout_secs: u64) -> Self {
        Self {
            timeout_secs,
            ..Self::new()
        }
    }

    /// Create a tool that requests sandboxing by default.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool with these execution defaults.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_bash_tool::BashTool::sandboxed();
    /// ```
    pub fn sandboxed() -> Self {
        Self {
            timeout_secs: 120,
            sandboxed: true,
            default_cwd: None,
        }
    }
}

impl Default for BashTool {
    fn default() -> Self {
        Self::new()
    }
}
impl BashTool {
    /// Construct a tool with explicit execution defaults.
    ///
    /// # Arguments
    /// * `timeout_secs` - Time allowed for each command unless overridden.
    /// * `sandboxed` - Default sandbox request, subject to shared policy.
    /// * `default_cwd` - Workspace used when the call omits its directory.
    /// # Returns
    /// A tool with the supplied defaults.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_bash_tool::BashTool::configured(30, true, None);
    /// ```
    pub fn configured(timeout_secs: u64, sandboxed: bool, default_cwd: Option<PathBuf>) -> Self {
        Self {
            timeout_secs,
            sandboxed,
            default_cwd,
        }
    }
}
