//! Application configuration adapter for shared introduction delivery.
use anyhow::Result;

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
/// use codetether_agent::a2a::intro::send::send_intro;
///
/// send_intro(
///     "http://192.0.2.10:4317",
///     "reviewer",
///     "http://192.0.2.20:4317",
///     Some("discovered-capability"),
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
) -> Result<()> {
    codetether_a2a_http::intro::send::send_intro(
        endpoint,
        agent_name,
        self_url,
        peer_token,
        super::ledger::ledger_path,
    )
    .await
}
