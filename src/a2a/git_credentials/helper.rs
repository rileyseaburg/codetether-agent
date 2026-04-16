//! Git credential helper entrypoint logic.
//!
//! This module is the runtime bridge between Git's credential protocol and the
//! control-plane credential broker.
//!
//! # Examples
//!
//! ```ignore
//! run_git_credential_helper(&args).await?;
//! ```

use anyhow::{anyhow, Result};

use super::gh_cli::{emit_credentials_via_gh_cli, should_delegate_to_gh_cli};
use super::request_git_credentials;
use super::stdin_query::read_git_credential_query_from_stdin;

/// Runs the Git credential helper command for this worker binary.
///
/// `store` and `erase` are treated as no-ops because the worker only serves
/// short-lived `get` requests.
///
/// # Examples
///
/// ```ignore
/// run_git_credential_helper(&args).await?;
/// ```
pub async fn run_git_credential_helper(
    args: &crate::cli::GitCredentialHelperArgs,
) -> Result<()> {
    let operation = args.operation.as_deref().unwrap_or("get").trim();
    if matches!(operation, "store" | "erase") {
        return Ok(());
    }
    let query = read_git_credential_query_from_stdin()?;
    let server = args
        .server
        .clone()
        .or_else(|| std::env::var("CODETETHER_SERVER").ok())
        .ok_or_else(|| {
            anyhow!("CODETETHER_SERVER is not set for Git credential helper")
        })?;
    let token = args
        .token
        .clone()
        .or_else(|| std::env::var("CODETETHER_TOKEN").ok());
    let worker_id = args
        .worker_id
        .clone()
        .or_else(|| std::env::var("CODETETHER_WORKER_ID").ok());
    let creds = request_git_credentials(
        &server,
        &token,
        worker_id.as_deref(),
        &args.workspace_id,
        operation,
        &query,
    )
    .await?;
    if let Some(creds) = creds {
        if should_delegate_to_gh_cli(&query, &creds) {
            match emit_credentials_via_gh_cli(&query, &creds) {
                Ok(()) => return Ok(()),
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        "gh CLI delegation failed, falling back to direct credential output"
                    );
                }
            }
        }
        println!("username={}", creds.username);
        println!("password={}", creds.password);
        println!();
    }
    Ok(())
}
