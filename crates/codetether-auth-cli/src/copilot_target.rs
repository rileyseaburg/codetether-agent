//! Select public or enterprise GitHub authentication destination.
use crate::DEFAULT_GITHUB_DOMAIN;
use anyhow::Result;
use codetether_provider_copilot::normalize_enterprise_domain;

pub(super) fn select(
    enterprise_url: Option<String>,
) -> Result<(&'static str, String, Option<String>)> {
    let (provider_id, domain, enterprise_domain) = match enterprise_url {
        Some(raw) => {
            let domain = normalize_enterprise_domain(&raw);
            if domain.is_empty() {
                anyhow::bail!("--enterprise-url cannot be empty");
            }
            ("github-copilot-enterprise", domain.clone(), Some(domain))
        }
        None => ("github-copilot", DEFAULT_GITHUB_DOMAIN.to_string(), None),
    };

    Ok((provider_id, domain, enterprise_domain))
}
