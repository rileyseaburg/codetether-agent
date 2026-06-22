//! Minimal `--sso URL` profile discovery.
//!
//! Finds an existing AWS CLI SSO profile whose start URL matches the given
//! portal URL, including profiles that reference an `[sso-session ...]` block.

use super::aws_paths;
use super::sso_parse::{Sections, parse_sections};
use anyhow::{Context, Result};

/// Resolved AWS profile and optional default Bedrock region.
#[derive(Clone)]
pub(super) struct SsoProfile {
    pub profile: String,
    pub region: Option<String>,
}

/// Resolve a user-provided SSO URL to an existing AWS CLI profile.
pub(super) fn resolve(url: &str) -> Result<SsoProfile> {
    let target = normalize(url);
    let text =
        std::fs::read_to_string(aws_paths::config_path()?).context("Failed to read ~/.aws/config")?;
    let sections = parse_sections(&text);
    find_match(&sections, &target).ok_or_else(|| {
        anyhow::anyhow!(
            "No AWS SSO profile found for {target}. Run `aws configure sso` once for that URL."
        )
    })
}

/// Normalize portal URLs by removing fragments, queries, and trailing slash.
pub(super) fn normalize(url: &str) -> String {
    let clean = url
        .split('#')
        .next()
        .unwrap_or(url)
        .split('?')
        .next()
        .unwrap_or(url);
    clean.trim_end_matches('/').to_string()
}

fn find_match(sections: &Sections, target: &str) -> Option<SsoProfile> {
    sections.iter().find_map(|(name, values)| {
        let profile = name.strip_prefix("profile ")?;
        let direct = values
            .get("sso_start_url")
            .is_some_and(|v| normalize(v) == target);
        let via_session = values
            .get("sso_session")
            .and_then(|s| sections.get(&format!("sso-session {s}")))
            .and_then(|s| s.get("sso_start_url"))
            .is_some_and(|v| normalize(v) == target);
        (direct || via_session).then(|| SsoProfile {
            profile: profile.to_string(),
            region: values.get("region").cloned(),
        })
    })
}
