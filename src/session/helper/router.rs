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
