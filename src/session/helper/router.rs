use crate::provider::Message;
use crate::tool::ToolRegistry;
use serde_json::json;
use std::path::Path;

use super::text::{extract_candidate_file_paths, latest_user_text, truncate_with_ellipsis};

pub async fn build_proactive_lsp_context_message(
    _selected_provider: &str,
    step: usize,
    tool_registry: &ToolRegistry,
    session_messages: &[Message],
    workspace_dir: &Path,
) -> Option<Message> {
    if step != 1 {
        return None;
    }

    let Some(lsp_tool) = tool_registry.get("lsp") else {
        return None;
    };

    let Some(user_text) = latest_user_text(session_messages) else {
        return None;
    };

    let max_files = std::env::var("CODETETHER_PROACTIVE_LSP_MAX_FILES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(3);

    let max_chars = std::env::var("CODETETHER_PROACTIVE_LSP_MAX_CHARS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(1600);

    let paths = extract_candidate_file_paths(&user_text, workspace_dir, max_files);
    if paths.is_empty() {
        return None;
    }

    let mut sections: Vec<String> = Vec::new();
    for path in paths {
        let diagnostics_args = json!({
            "action": "diagnostics",
            "file_path": path,
        });

        match lsp_tool.execute(diagnostics_args).await {
            Ok(result) if result.success => {
                let output = result.output.trim();
                if !output.is_empty() && output != "No diagnostics found" {
                    sections.push(format!(
                        "File: {}\n{}",
                        path,
                        truncate_with_ellipsis(output, max_chars)
                    ));
                    continue;
                }
            }
            Ok(result) => {
                tracing::debug!(
                    file = %path,
                    output = %truncate_with_ellipsis(&result.output, 200),
                    "Proactive LSP diagnostics skipped file due to unsuccessful result"
                );
            }
            Err(e) => {
                tracing::debug!(file = %path, error = %e, "Proactive LSP diagnostics prefetch failed");
            }
        }

        let symbol_args = json!({
            "action": "documentSymbol",
            "file_path": path,
        });
        match lsp_tool.execute(symbol_args).await {
            Ok(result) if result.success => {
                sections.push(format!(
                    "File: {}\n{}",
                    path,
                    truncate_with_ellipsis(&result.output, max_chars / 2)
                ));
            }
            Ok(result) => {
                tracing::debug!(
                    file = %path,
                    output = %truncate_with_ellipsis(&result.output, 200),
                    "Proactive LSP symbol recovery skipped file due to unsuccessful result"
                );
            }
            Err(e) => {
                tracing::debug!(file = %path, error = %e, "Proactive LSP symbol recovery failed");
            }
        }
    }

    if sections.is_empty() {
        return None;
    }

    Some(Message {
        role: crate::provider::Role::System,
        content: vec![crate::provider::ContentPart::Text {
            text: format!(
                "Mandatory proactive LSP context (prefetched before first reply). Prioritize these real LSP diagnostics and errors over speculation. Do not call the lsp tool just to rediscover the same issues unless you need deeper navigation detail.\n\n{}",
                sections.join("\n\n---\n\n")
            ),
        }],
    })
}

pub fn known_good_router_candidates(provider: &str, failed_model: &str) -> Vec<String> {
    let failed = failed_model.trim();
    let mut candidates: Vec<String> = match provider {
        "openrouter" => vec![
            "openrouter/qwen/qwen3-coder:free".to_string(),
            "openrouter/openai/gpt-oss-120b:free".to_string(),
            "openrouter/google/gemma-3-27b-it:free".to_string(),
            "openrouter/meta-llama/llama-3.3-70b-instruct:free".to_string(),
        ],
        "zai" => vec!["zai/glm-5".to_string()],
        "glm5" => vec!["glm5/glm-5".to_string()],
        "github-copilot" | "github-copilot-enterprise" => {
            vec![format!("{provider}/gpt-5-mini")]
        }
        "openai-codex" => vec!["openai-codex/gpt-5-mini".to_string()],
        "gemini-web" => vec!["gemini-web/gemini-2.5-flash".to_string()],
        "local_cuda" => vec!["local_cuda/qwen3.5-9b".to_string()],
        "google" => vec!["google/gemini-2.5-flash".to_string()],
        "anthropic" => vec!["anthropic/claude-3-5-haiku-latest".to_string()],
        _ => Vec::new(),
    };

    candidates.retain(|candidate| {
        !candidate.eq_ignore_ascii_case(failed)
            && candidate
                .split('/')
                .next_back()
                .map(|model| !model.eq_ignore_ascii_case(failed))
                .unwrap_or(true)
    });
    candidates
}

pub fn choose_router_target(
    registry: &crate::provider::ProviderRegistry,
    selected_provider: &str,
    current_model: &str,
) -> Option<(String, String)> {
    let current_provider = selected_provider.to_ascii_lowercase();

    for candidate in known_good_router_candidates(selected_provider, current_model) {
        let (provider_name, model_name) = crate::provider::parse_model_string(&candidate);
        let provider_name = provider_name.unwrap_or(selected_provider);
        if registry.get(provider_name).is_some() {
            return Some((provider_name.to_string(), model_name.to_string()));
        }
    }

    if current_provider != "zai" && registry.get("zai").is_some() {
        return Some(("zai".to_string(), "glm-5".to_string()));
    }
    if current_provider != "glm5" && registry.get("glm5").is_some() {
        return Some(("glm5".to_string(), "glm-5".to_string()));
    }
    if current_provider != "openrouter" && registry.get("openrouter").is_some() {
        return Some((
            "openrouter".to_string(),
            "openai/gpt-oss-120b:free".to_string(),
        ));
    }

    None
}

/// CADMAS-CTX-aware variant of [`choose_router_target`] (Phase C step 17).
///
/// When `state.config.enabled` is `true`, the rule-based candidate list
/// from [`known_good_router_candidates`] is re-ordered by the LCB score
/// `μ − γ·√u` under the supplied `bucket`. Candidates with no
/// observations yet keep their original rule order (cold-start
/// conservatism). When `state.config.enabled` is `false`, this function
/// is exactly [`choose_router_target`].
///
/// The actual outcome update — `state.update(provider, "model_call",
/// bucket, success)` — lives in the prompt loop's retry handler and is
/// wired in a follow-up commit. This function is the *selection* half
/// of the bandit loop; the *update* half is the hook point.
///
/// # Arguments
///
/// * `registry` — Active provider registry (same as the non-bandit path).
/// * `state` — CADMAS-CTX sidecar for this session. Read-only here.
/// * `bucket` — Context bucket extracted from the current turn's
///   [`RelevanceMeta`](crate::session::relevance::RelevanceMeta).
/// * `selected_provider` / `current_model` — Same semantics as
///   [`choose_router_target`].
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::provider::ProviderRegistry;
/// use codetether_agent::session::delegation::{DelegationConfig, DelegationState};
/// use codetether_agent::session::helper::router::choose_router_target_bandit;
/// use codetether_agent::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};
///
/// let registry = ProviderRegistry::from_vault().await.unwrap();
/// let state = DelegationState::with_config(DelegationConfig::default());
/// let bucket = Bucket {
///     difficulty: Difficulty::Easy,
///     dependency: Dependency::Isolated,
///     tool_use: ToolUse::No,
/// };
/// let _ = choose_router_target_bandit(&registry, &state, bucket, "openai", "gpt-5");
/// # });
/// ```
pub fn choose_router_target_bandit(
    registry: &crate::provider::ProviderRegistry,
    state: &crate::session::delegation::DelegationState,
    bucket: crate::session::relevance::Bucket,
    selected_provider: &str,
    current_model: &str,
) -> Option<(String, String)> {
    if !state.config.enabled {
        return choose_router_target(registry, selected_provider, current_model);
    }

    // Enumerate rule-based candidates, annotate each with an LCB score
    // (or 0.0 when there is no posterior yet), sort stably by score
    // descending, and return the first one the registry knows about.
    let candidates = known_good_router_candidates(selected_provider, current_model);
    let mut scored: Vec<(String, f64)> = candidates
        .into_iter()
        .map(|raw| {
            let (provider_name, _model_name) = crate::provider::parse_model_string(&raw);
            let provider_name = provider_name.unwrap_or(selected_provider);
            let score = state
                .score(provider_name, "model_call", bucket)
                .unwrap_or(0.0);
            (raw, score)
        })
        .collect();
    scored.sort_by(|a, b| {
        b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
    });

    for (candidate, _score) in scored {
        let (provider_name, model_name) = crate::provider::parse_model_string(&candidate);
        let provider_name = provider_name.unwrap_or(selected_provider);
        if registry.get(provider_name).is_some() {
            return Some((provider_name.to_string(), model_name.to_string()));
        }
    }

    // No re-ordered candidate was registered — fall back to the
    // original rule-based ladder so we never regress against the
    // non-bandit behaviour.
    choose_router_target(registry, selected_provider, current_model)
}
