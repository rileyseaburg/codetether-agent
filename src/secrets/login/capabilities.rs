//! Refuse administrator capabilities before a token can be saved.
use super::facts::Envelope;
use anyhow::{Result, ensure};
use std::collections::HashMap;
const ADMIN_PATHS: [&str; 4] = [
    "sys/auth",
    "sys/policies/acl",
    "sys/mounts",
    "auth/token/create",
];

pub(super) async fn check(client: &reqwest::Client, address: &str, token: &str) -> Result<()> {
    let capabilities: Envelope<HashMap<String, Vec<String>>> = super::http::json(
        client
            .post(format!("{address}/v1/sys/capabilities-self"))
            .header("X-Vault-Token", token)
            .json(&serde_json::json!({"paths": ADMIN_PATHS})),
    )
    .await?;
    for path in ADMIN_PATHS {
        let permissions = capabilities
            .data
            .get(path)
            .ok_or_else(|| anyhow::anyhow!("Vault did not report required capability checks"))?;
        ensure!(
            !permissions
                .iter()
                .any(|p| ["sudo", "root", "create", "update", "delete", "patch"]
                    .contains(&p.as_str())),
            "Vault token has administrator capabilities; use an app-scoped role"
        );
    }
    Ok(())
}
