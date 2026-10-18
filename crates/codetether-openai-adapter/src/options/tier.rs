//! Apply validated Codex service-tier suffixes.
use codetether_provider_core::codex::service_tier_catalog;
pub(super) fn apply(tier: Option<&str>, base: &str, bare: &str) -> Result<String, String> {
    Ok(match tier {
        None => base.to_string(),
        Some("default" | "auto" | "flex") => bare.to_string(),
        Some(t @ ("fast" | "priority" | "ultrafast")) => {
            let suffix = if t == "ultrafast" {
                "-ultrafast"
            } else {
                "-fast"
            };
            if !service_tier_catalog::suffixes(bare).contains(&suffix) {
                return Err(format!(
                    "Model `{bare}` does not support `service_tier` `{t}`"
                ));
            }
            format!("{bare}{suffix}")
        }
        Some(other) => return Err(format!("Unsupported `service_tier` `{other}`")),
    })
}
