//! Loaded-instance budgets for local sessions; never assume a catalog maximum.
use anyhow::{Context, Result, ensure};
use codetether_context_policy::derivation::{DerivePolicy, dispatch::Execution};
use codetether_provider_core::Provider;

pub(super) async fn execution(
    provider: &dyn Provider,
    model: &str,
    policy: DerivePolicy,
    fixed_tokens: usize,
) -> Result<Option<Execution>> {
    if provider.name() != "lmstudio" && provider.name() != "lm-studio" {
        return Ok(None);
    }
    let models = provider.list_models().await?;
    let capacity = models
        .iter()
        .find(|entry| entry.id == model)
        .with_context(|| format!("LM Studio model '{model}' is not loaded; cannot budget context"))?
        .context_window;
    let budget = input_budget(capacity, policy);
    ensure!(
        budget > 0,
        "LM Studio loaded context is too small for a safe request"
    );
    ensure!(
        fixed_tokens < budget,
        "LM Studio context budget exceeded: fixed instructions, tool schemas and pinned constraints need {fixed_tokens} tokens, safe input budget is {budget} (loaded capacity {capacity}); reduce optional tools or use a larger loaded context"
    );
    Ok(Some(Execution::Incremental {
        budget_tokens: budget,
    }))
}

fn input_budget(capacity: usize, policy: DerivePolicy) -> usize {
    // Leave half the loaded window for estimation error, output and framing.
    // The derivation accounts for system instructions, schemas and pinned text
    // before selecting history. A provider-boundary guard rechecks after reloads.
    let safe = (capacity / 2).min(capacity.saturating_sub(2048));
    match policy {
        DerivePolicy::Incremental { budget_tokens } if budget_tokens > 0 => safe.min(budget_tokens),
        _ => safe,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loaded_capacity_and_smaller_explicit_budget_win() {
        assert_eq!(input_budget(32768, DerivePolicy::Legacy), 16384);
        assert_eq!(input_budget(4608, DerivePolicy::Legacy), 2304);
        assert_eq!(input_budget(1024, DerivePolicy::Legacy), 0);
        assert_eq!(
            input_budget(
                32768,
                DerivePolicy::Incremental {
                    budget_tokens: 1000
                }
            ),
            1000
        );
    }
}
