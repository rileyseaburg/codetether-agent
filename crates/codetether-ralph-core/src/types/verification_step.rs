//! VerificationStep serializable contract.
use super::*;

/// A concrete verification step that must pass for a story to be marked complete.
///
/// This complements `quality_checks` (which are repo/toolchain-level gates).
/// Verification steps are story-level and can validate user-visible outcomes
/// like E2E flows, artifacts (e.g. Cypress videos), or deployment endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
///
/// # Examples
/// ```
/// let step = codetether_ralph_core::VerificationStep::FileExists { name: None, path: "output.txt".into(), glob: false };
/// match step { codetether_ralph_core::VerificationStep::FileExists { path, .. } => assert_eq!(path, "output.txt"), _ => unreachable!() }
/// ```
pub enum VerificationStep {
    /// Run a shell command and optionally assert on output and/or produced files.
    Shell {
        /// Optional label for UI/logging.
        #[serde(default)]
        name: Option<String>,

        /// Shell command to run.
        command: String,

        /// Optional working directory (relative to repo root / check root).
        #[serde(default)]
        cwd: Option<String>,

        /// Substrings that must be present in stdout+stderr.
        #[serde(default)]
        expect_output_contains: Vec<String>,

        /// File globs that must match at least one file after the command runs.
        /// Example: "cypress/videos/**/*.mp4"
        #[serde(default)]
        expect_files_glob: Vec<String>,
    },

    /// Ensure a file (or glob pattern) exists.
    FileExists {
        #[serde(default)]
        name: Option<String>,
        /// Path or glob pattern.
        path: String,
        /// If true, treat `path` as a glob.
        #[serde(default)]
        glob: bool,
    },

    /// Validate an HTTP endpoint is reachable (useful for “deployment is live”).
    Url {
        #[serde(default)]
        name: Option<String>,
        url: String,
        /// HTTP method (default: GET).
        #[serde(default = "default_http_method")]
        method: String,
        /// Expected status (default: 200).
        #[serde(default = "default_http_status")]
        expect_status: u16,
        /// Substrings that must appear in the response body.
        #[serde(default)]
        expect_body_contains: Vec<String>,
        /// Timeout in seconds (default: 30).
        #[serde(default = "default_http_timeout_secs")]
        timeout_secs: u64,
    },
}
