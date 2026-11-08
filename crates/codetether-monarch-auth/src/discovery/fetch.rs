//! Fetch the two pinned OAuth discovery documents before validation.
use super::{
    ISSUER, json,
    models::{ResourceMetadata, ServerMetadata},
    validate::validate,
};
use anyhow::{Result, anyhow};
use reqwest::Client;

pub(crate) async fn discover(client: &Client) -> Result<()> {
    let resource = client
        .get(format!("{ISSUER}/.well-known/oauth-protected-resource/mcp"))
        .send()
        .await
        .map_err(|_| anyhow!("Monarch resource discovery failed"))?;
    let resource: ResourceMetadata = json(resource).await?;
    let server = client
        .get(format!("{ISSUER}/.well-known/oauth-authorization-server"))
        .send()
        .await
        .map_err(|_| anyhow!("Monarch authorization discovery failed"))?;
    validate(&resource, &json::<ServerMetadata>(server).await?)
}
