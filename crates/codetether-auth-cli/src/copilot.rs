//! Authentication copilot responsibilities.
use crate::{CopilotAuthArgs, poll_for_access_token, request_device_code, resolve_client_id};
use anyhow::Result;
use codetether_secrets as secrets;
use reqwest::Client;

pub(super) async fn authenticate_copilot(
    args: CopilotAuthArgs,
    application_version: &str,
) -> Result<()> {
    if secrets::secrets_manager().is_none() {
        anyhow::bail!(
            "HashiCorp Vault is not configured. Set VAULT_ADDR and VAULT_TOKEN before running `codetether auth copilot`."
        );
    }

    let (provider_id, domain, enterprise_domain) =
        crate::copilot_target::select(args.enterprise_url)?;

    let client = Client::new();
    let client_id = resolve_client_id(args.client_id)?;
    let user_agent = format!("codetether-agent/{}", application_version);
    let device = request_device_code(&client, &domain, &user_agent, &client_id).await?;

    crate::copilot_instructions::show(&device);

    let token = poll_for_access_token(&client, &domain, &user_agent, &client_id, &device).await?;

    crate::copilot_save::save(provider_id, enterprise_domain, token).await
}
