//! Git credential query parsing from stdin.
//!
//! Git feeds credential helpers a simple `key=value` stream on stdin. This
//! module parses that stream into the strongly typed query struct.
//!
//! # Examples
//!
//! ```ignore
//! let query = read_git_credential_query_from_stdin()?;
//! ```

use anyhow::{Context, Result};
use std::io::{self, Read};

use super::GitCredentialQuery;

/// Reads and parses a Git credential-helper query from stdin.
///
/// Unknown keys are ignored so the helper stays compatible with future Git
/// protocol additions.
///
/// # Examples
///
/// ```ignore
/// let query = read_git_credential_query_from_stdin()?;
/// ```
pub(super) fn read_git_credential_query_from_stdin() -> Result<GitCredentialQuery> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).context("Failed to read Git credential request from stdin")?;
    let mut query = GitCredentialQuery::default();
    for line in input.lines() {
        let Some((key, value)) = line.split_once('=') else { continue; };
        if value.trim().is_empty() {
            continue;
        }
        {
            match key.trim() {
                "protocol" => query.protocol = Some(value.trim().to_string()),
                "host" => query.host = Some(value.trim().to_string()),
                "path" => query.path = Some(value.trim().to_string()),
                _ => {}
            }
        }
    }
    Ok(query)
}
