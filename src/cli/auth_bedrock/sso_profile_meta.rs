//! AWS config metadata needed to reuse an SSO refresh token.

use super::aws_paths;
use super::sso_parse::parse_sections;
use anyhow::{Context, Result};

/// SSO profile fields required for future role-credential refreshes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SsoProfileMeta {
    pub profile: String,
    pub start_url: String,
    pub sso_region: String,
    pub account_id: Option<String>,
    pub role_name: Option<String>,
}

/// Resolve SSO metadata for an AWS profile, if it is SSO-backed.
pub(super) fn resolve(profile: &str) -> Result<Option<SsoProfileMeta>> {
    let text =
        std::fs::read_to_string(aws_paths::config_path()?).context("Failed to read ~/.aws/config")?;
    Ok(from_config_text(profile, &text))
}

pub(super) fn from_config_text(profile: &str, text: &str) -> Option<SsoProfileMeta> {
    let sections = parse_sections(text);
    let values = sections.get(&profile_section(profile))?;
    let session = values
        .get("sso_session")
        .and_then(|s| sections.get(&format!("sso-session {s}")));
    let start_url = values
        .get("sso_start_url")
        .or_else(|| session.and_then(|s| s.get("sso_start_url")))?
        .clone();
    let sso_region = values
        .get("sso_region")
        .or_else(|| session.and_then(|s| s.get("sso_region")))?
        .clone();
    Some(SsoProfileMeta {
        profile: profile.to_string(),
        start_url,
        sso_region,
        account_id: values.get("sso_account_id").cloned(),
        role_name: values.get("sso_role_name").cloned(),
    })
}

fn profile_section(profile: &str) -> String {
    match profile {
        "default" => "default".to_string(),
        name => format!("profile {name}"),
    }
}
