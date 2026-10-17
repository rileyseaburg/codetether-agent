//! Outbound mesh introduction: one-shot, tagged, ledger-deduplicated.

use anyhow::Result;

use super::ledger;
use std::path::PathBuf;

/// Send a non-blocking intro `message/send` to `endpoint`, unless the
/// persistent ledger says we already introduced ourselves there.
///
/// The message carries `metadata[codetether.intro] = true` so receivers
/// can short-circuit it without an LLM call.
///
/// # Arguments
///
/// * `endpoint` - Remote A2A HTTP endpoint.
/// * `agent_name` - Local agent name included in the greeting.
/// * `self_url` - Local public URL included in the greeting.
/// * `peer_token` - Capability learned from first-party discovery, when present.
/// * `resolve_path` - Resolve the current ledger path on each storage operation.
///
/// # Returns
///
/// Returns after delivery succeeds or a matching introduction is already recorded.
///
/// # Errors
///
/// Returns an error when all bounded delivery attempts fail.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_a2a_http::intro::send::send_intro;
///
/// send_intro(
///     "http://192.0.2.10:4317",
///     "reviewer",
///     "http://192.0.2.20:4317",
///     Some("discovered-capability"),
///     || Some(std::path::PathBuf::from("introduced.json")),
/// )
/// .await?;
/// # Ok::<(), anyhow::Error>(())
/// # });
/// ```
pub async fn send_intro(
    endpoint: &str,
    agent_name: &str,
    self_url: &str,
    peer_token: Option<&str>,
    mut resolve_path: impl FnMut() -> Option<PathBuf>,
) -> Result<()> {
    let identity = super::identity::key(endpoint, peer_token);
    if ledger::contains(&identity, resolve_path()) {
        tracing::debug!(target: "codetether_agent::a2a::intro::send", peer = %endpoint, "Skipping intro: already in ledger");
        return Ok(());
    }

    let token = peer_token
        .map(ToString::to_string)
        .or_else(|| std::env::var("CODETETHER_AUTH_TOKEN").ok());
    let payload = super::payload::build(agent_name, self_url);
    super::deliver::send(endpoint, token.as_deref(), payload).await?;
    ledger::record(&identity, &mut resolve_path);
    tracing::info!(target: "codetether_agent::a2a::intro::send", peer = %endpoint, "Auto-intro message sent");
    Ok(())
}
