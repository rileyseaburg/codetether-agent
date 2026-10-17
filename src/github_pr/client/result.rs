//! CLI-facing PR outcome with presentation metadata.
use super::*;

#[derive(Debug, Serialize)]
/// PR outcome with the CLI's head identity and output-format preference.
pub struct PullRequestResult {
    pub number: u64,
    pub html_url: String,
    pub state: String,
    pub draft: bool,
    pub head_sha: String,
    pub json: bool,
}
