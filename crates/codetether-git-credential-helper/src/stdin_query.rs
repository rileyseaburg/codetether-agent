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
    io::stdin()
        .read_to_string(&mut input)
        .context("Failed to read Git credential request from stdin")?;
    Ok(codetether_a2a_http::git_credentials::parse_query(&input))
}
