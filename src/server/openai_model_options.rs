//! Fold OpenAI-style `reasoning_effort` and `service_tier` request fields
//! into the provider's model-suffix syntax (`model-ultrafast:high`).
//!
//! Only `openai-codex` understands these suffixes. Values are validated
//! against its catalogs so unsupported requests fail with a clear 400
//! instead of becoming an unknown model id.

use crate::provider::openai_codex::{reasoning_catalog, service_tier_catalog};

/// Apply optional `service_tier` and `reasoning_effort` to a model id.
///
/// # Arguments
///
/// * `provider` — Selected provider name.
/// * `model` — Provider-relative model id, possibly already suffixed.
/// * `service_tier` — `fast`/`priority`, `ultrafast`, or `default`/`auto`/`flex`.
/// * `reasoning_effort` — Effort level such as `high` or `ultra`.
///
/// # Returns
///
/// The suffixed model id, unchanged when neither field is set.
///
/// # Errors
///
/// Returns a client-facing message when the provider or model does not
/// support the requested tier or effort.
pub(super) fn apply(
    provider: &str,
    model: &str,
    service_tier: Option<&str>,
    reasoning_effort: Option<&str>,
) -> Result<String, String> {
    let tier = service_tier.map(str::trim).filter(|t| !t.is_empty());
    let effort = reasoning_effort.map(str::trim).filter(|e| !e.is_empty());
    if tier.is_none() && effort.is_none() {
        return Ok(model.to_string());
    }
    if provider != "openai-codex" {
        return Err(format!(
            "`service_tier`/`reasoning_effort` are not supported for provider `{provider}`"
        ));
    }
    let (base, suffix_effort) = model
        .rsplit_once(':')
        .map_or((model, None), |(b, l)| (b, Some(l)));
    let bare = base
        .strip_suffix("-ultrafast")
        .or_else(|| base.strip_suffix("-fast"))
        .unwrap_or(base);
    let base = match tier {
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
    };
    match effort.or(suffix_effort) {
        None => Ok(base),
        Some(level) if reasoning_catalog::supports(bare, level) => Ok(format!("{base}:{level}")),
        Some(level) => Err(format!(
            "Model `{bare}` does not support `reasoning_effort` `{level}`"
        )),
    }
}

#[cfg(test)]
#[path = "openai_model_options_tests.rs"]
mod tests;
