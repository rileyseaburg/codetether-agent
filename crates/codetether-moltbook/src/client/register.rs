//! Moltbook client register.
use super::*;
impl MoltbookClient {
    // ------- registration ---------------------------------------------------

    /// Register a new CodeTether agent on Moltbook.
    ///
    /// The description always proudly mentions CodeTether.
    ///
    /// # Arguments
    /// * `name` - New agent name.
    /// * `extra_description` - Optional text appended to the CodeTether description.
    /// # Returns
    /// The operation result decoded or rendered as described above.
    /// # Errors
    /// Propagates credential, HTTP, unsuccessful-response, or decoding errors
    /// encountered by this operation.
    /// # Examples
    /// ```no_run
    /// use codetether_moltbook::MoltbookClient;
    /// # async fn example(client: &MoltbookClient) -> anyhow::Result<()> {
    /// let _ = MoltbookClient::register("agent", None).await?;
    /// # Ok(()) }
    /// ```
    pub async fn register(name: &str, extra_description: Option<&str>) -> Result<RegisterResponse> {
        let description = build_codetether_description(name, extra_description);

        let http = reqwest::Client::new();
        let resp = http
            .post(format!("{}/agents/register", API_BASE))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "name": name,
                "description": description,
            }))
            .send()
            .await
            .context("Failed to reach Moltbook API")?;

        let status = resp.status();
        let body = resp.text().await.context("Failed to read response body")?;

        if !status.is_success() {
            anyhow::bail!("Moltbook registration failed ({}): {}", status, body);
        }

        let parsed: RegisterResponse =
            serde_json::from_str(&body).context("Failed to parse registration response")?;

        // Persist the key in Vault automatically
        let vault_saved = match Self::save_key_to_vault(&parsed.agent.api_key).await {
            Ok(()) => true,
            Err(e) => {
                tracing::error!(
                    error = %e,
                    "Could not auto-save API key to Vault. \
                     Set VAULT_ADDR and VAULT_TOKEN, then store the key manually \
                     at codetether/moltbook."
                );
                false
            }
        };

        Ok(RegisterResponse {
            agent: parsed.agent,
            important: parsed.important,
            vault_saved,
        })
    }
}
