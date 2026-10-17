//! Generate a PRD with an initial request and up to two JSON repair attempts.

use anyhow::{Context, Result};
use codetether_okr::Okr;
use codetether_provider_core::Provider;
use codetether_ralph_core::Prd;
/// Generate a PRD from a task and its objective key results.
///
/// # Arguments
/// * `task` — Task description.
/// * `okr` — Objective whose key results shape the requested stories.
/// * `provider` — Configured provider for all attempts.
/// * `model` — Model name used in every completion request.
///
/// # Returns
/// A parsed PRD with timestamps and normalized quality checks.
///
/// # Errors
/// Returns provider errors immediately, or a parsing error after three attempts.
///
/// # Panics
/// Preserves the existing byte-based 2,000-byte repair excerpt; a non-boundary cut can panic.
///
/// # Examples
/// ```no_run
/// async fn generate(provider: &dyn codetether_provider_core::Provider, okr: &codetether_okr::Okr)
///     -> anyhow::Result<codetether_ralph_core::Prd> {
///     codetether_go_prd::generate_prd_from_task("Implement feature", okr, provider, "model").await
/// }
/// ```
pub async fn generate_prd_from_task(
    task: &str,
    okr: &Okr,
    provider: &dyn Provider,
    model: &str,
) -> Result<Prd> {
    let prompt = crate::prompt::build(task, okr);
    let request = crate::request::initial(&prompt, model);
    let mut last_text = String::new();
    let mut last_error = String::new();
    for attempt in 0..3 {
        let req = if attempt == 0 {
            request.clone()
        } else {
            crate::repair_request::build(&prompt, model, &last_text, &last_error, attempt)
        };
        let response = provider
            .complete(req)
            .await
            .context("Failed to generate PRD from LLM")?;
        last_text = crate::response_text::collect(&response);
        match crate::parse::response(&last_text) {
            Ok(mut prd) => {
                if attempt > 0 {
                    tracing::info!(target:"codetether_agent::cli::go_ralph",attempt,"PRD JSON repair succeeded");
                }
                crate::normalize::apply(&mut prd);
                return Ok(prd);
            }
            Err(error) => last_error = error,
        }
    }
    anyhow::bail!("Failed to extract valid PRD JSON after 3 attempts. Last error: {last_error}");
}
