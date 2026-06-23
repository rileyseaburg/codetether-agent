//! Default SSO profile discovery when no profile/URL was supplied.

use super::aws_paths;
use super::sso::SsoProfile;
use super::sso_parse::parse_sections;
use anyhow::{Context, Result, bail};

/// Pick the only configured AWS SSO profile, or ask for a disambiguator.
pub(super) fn resolve() -> Result<SsoProfile> {
    let config = aws_paths::config_path()?;
    if !config.exists() {
        bail!(
            "No AWS config found at {}. This command runs `aws sso login`, which needs a \
             named SSO profile. Create one with `aws configure sso` (device-code login \
             still happens here via --device-code), then pass --profile <name> or --sso <start-url>.",
            config.display()
        );
    }
    let text = std::fs::read_to_string(&config)
        .with_context(|| format!("Failed to read {}", config.display()))?;
    let matches: Vec<SsoProfile> = parse_sections(&text)
        .into_iter()
        .filter_map(|(name, values)| {
            let profile = name.strip_prefix("profile ")?.to_string();
            let is_sso = values.contains_key("sso_session") || values.contains_key("sso_start_url");
            is_sso.then(|| SsoProfile {
                profile,
                region: values.get("region").cloned(),
            })
        })
        .collect();
    match matches.as_slice() {
        [only] => Ok(only.clone()),
        [] => bail!("No SSO profile found. Use --sso <start-url> or run `aws configure sso`."),
        many => bail!(
            "Multiple SSO profiles found ({}). Use --sso <start-url> or --profile <name>.",
            many.len()
        ),
    }
}
