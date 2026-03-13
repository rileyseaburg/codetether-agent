use super::SessionEvent;
use super::helper::build::{
    build_request_requires_tool, looks_like_build_execution_request,
    should_force_build_tool_first_retry,
};
use super::helper::edit::detect_stub_in_tool_input;
use super::helper::edit::normalize_tool_call_for_execution;
use super::helper::markup::normalize_textual_tool_calls;
use super::helper::provider::{
    assistant_claims_imminent_tool_use, choose_default_provider,
    resolve_provider_for_session_request, should_retry_missing_native_tool_call,
};
use super::helper::runtime::is_codesearch_no_match_output;
use super::helper::stream::collect_stream_completion_with_events;
use super::helper::text::{extract_candidate_file_paths, extract_text_content};
use crate::provider::{
    CompletionResponse, ContentPart, FinishReason, Message, Role, StreamChunk, ToolDefinition,
    Usage,
};
use futures::stream;
use serde_json::json;
use std::fs;
use tempfile::tempdir;
use tokio::sync::mpsc;

#[tokio::test]
async fn collect_stream_completion_emits_incremental_text_chunks() {
    let (tx, mut rx) = mpsc::channel(8);
    let stream = Box::pin(stream::iter(vec![
        StreamChunk::Text("hel".to_string()),
        StreamChunk::Text("lo".to_string()),
        StreamChunk::Done {
            usage: Some(Usage {
                prompt_tokens: 3,
                completion_tokens: 5,
                total_tokens: 8,
                ..Default::default()
            }),
        },
    ]));

    let response = collect_stream_completion_with_events(stream, Some(&tx))
        .await
        .expect("stream should collect");

    let first = rx.recv().await.expect("first chunk");
    let second = rx.recv().await.expect("second chunk");

    assert!(matches!(first, SessionEvent::TextChunk(text) if text == "hel"));
    assert!(matches!(second, SessionEvent::TextChunk(text) if text == "hello"));
    assert_eq!(extract_text_content(&response.message.content), "hello");
    assert_eq!(response.usage.total_tokens, 8);
    assert_eq!(response.finish_reason, FinishReason::Stop);
}

#[tokio::test]
async fn collect_stream_completion_reconstructs_tool_calls() {
    let stream = Box::pin(stream::iter(vec![
        StreamChunk::ToolCallStart {
            id: "call_1".to_string(),
            name: "read".to_string(),
        },
        StreamChunk::ToolCallDelta {
            id: "call_1".to_string(),
            arguments_delta: "{\"path\":\"src/main.rs\"}".to_string(),
        },
        StreamChunk::ToolCallEnd {
            id: "call_1".to_string(),
        },
        StreamChunk::Done { usage: None },
    ]));

    let response = collect_stream_completion_with_events(stream, None)
        .await
        .expect("stream should collect");

    assert_eq!(response.finish_reason, FinishReason::ToolCalls);
    assert!(matches!(
        response.message.content.first(),
        Some(ContentPart::ToolCall { id, name, arguments, .. })
            if id == "call_1" && name == "read" && arguments == "{\"path\":\"src/main.rs\"}"
    ));
}

#[test]
fn normalize_textual_tool_calls_extracts_markup_into_structured_calls() {
    let response = CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "<tool_call>{\"name\":\"lsp\",\"arguments\":{\"action\":\"workspaceSymbol\",\"query\":\"Session\"}}</tool_call>".to_string(),
            }],
        },
        usage: Usage::default(),
        finish_reason: FinishReason::Stop,
    };
    let tools = vec![ToolDefinition {
        name: "lsp".to_string(),
        description: "LSP operations".to_string(),
        parameters: json!({"type":"object"}),
    }];

    let normalized = normalize_textual_tool_calls(response, &tools);
    assert_eq!(normalized.finish_reason, FinishReason::ToolCalls);
    assert!(
        normalized
            .message
            .content
            .iter()
            .any(|p| matches!(p, ContentPart::ToolCall { name, .. } if name == "lsp"))
    );
    assert!(
        normalized
            .message
            .content
            .iter()
            .all(|p| !matches!(p, ContentPart::Text { text } if text.contains("<tool_call>")))
    );
}

#[test]
fn normalize_textual_tool_calls_ignores_unknown_tools() {
    let response = CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "<tool_call>{\"name\":\"not_a_tool\",\"arguments\":{}}</tool_call>"
                    .to_string(),
            }],
        },
        usage: Usage::default(),
        finish_reason: FinishReason::Stop,
    };
    let tools = vec![ToolDefinition {
        name: "bash".to_string(),
        description: "shell".to_string(),
        parameters: json!({"type":"object"}),
    }];

    let normalized = normalize_textual_tool_calls(response, &tools);
    assert_eq!(normalized.finish_reason, FinishReason::Stop);
    assert!(
        normalized
            .message
            .content
            .iter()
            .all(|p| !matches!(p, ContentPart::ToolCall { .. }))
    );
}

#[test]
fn assistant_claims_imminent_tool_use_detects_minimax_style_prose() {
    let tools = vec![ToolDefinition {
        name: "bash".to_string(),
        description: "shell".to_string(),
        parameters: json!({"type":"object"}),
    }];

    assert!(assistant_claims_imminent_tool_use(
        "First, I'll use bash to inspect the repo and then patch the issue.",
        &tools,
    ));
    assert!(!assistant_claims_imminent_tool_use(
        "The fix is to update the parser and rerun tests.",
        &tools,
    ));
}

#[test]
fn should_retry_missing_native_tool_call_targets_minimax_only() {
    let tools = vec![ToolDefinition {
        name: "bash".to_string(),
        description: "shell".to_string(),
        parameters: json!({"type":"object"}),
    }];

    assert!(should_retry_missing_native_tool_call(
        "minimax",
        "MiniMax-M2.5",
        0,
        &tools,
        "I'll use bash to inspect the repository first.",
        false,
        1,
    ));
    assert!(!should_retry_missing_native_tool_call(
        "anthropic",
        "claude-sonnet-4",
        0,
        &tools,
        "I'll use bash to inspect the repository first.",
        false,
        1,
    ));
    assert!(!should_retry_missing_native_tool_call(
        "minimax",
        "MiniMax-M2.5",
        1,
        &tools,
        "I'll use bash to inspect the repository first.",
        false,
        1,
    ));
}

#[test]
fn explicit_provider_must_not_fallback_when_unavailable() {
    let providers = vec!["zai", "openai"];
    let result = resolve_provider_for_session_request(&providers, Some("local_cuda"));
    assert!(result.is_err());
    assert!(
        result
            .err()
            .map(|e| e.to_string())
            .unwrap_or_default()
            .contains("selected explicitly but is unavailable")
    );
}

#[test]
fn explicit_provider_is_used_when_available() {
    let providers = vec!["local_cuda", "zai"];
    let result = resolve_provider_for_session_request(&providers, Some("local_cuda"));
    assert_eq!(result.ok(), Some("local_cuda"));
}

#[test]
fn default_provider_prefers_openai() {
    let providers = vec!["zai", "openai"];
    assert_eq!(choose_default_provider(&providers), Some("openai"));
}

#[test]
fn extract_candidate_file_paths_filters_to_existing_files() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("api/src")).expect("mkdirs");
    fs::write(root.join("api/src/index.ts"), "export {};").expect("write file");

    let text = "Check api/src/index.ts and missing/path.ts";
    let paths = extract_candidate_file_paths(text, root, 5);
    assert_eq!(paths, vec!["api/src/index.ts".to_string()]);
}

#[test]
fn extract_candidate_file_paths_dedupes_and_respects_limit() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("a")).expect("mkdirs");
    fs::create_dir_all(root.join("b")).expect("mkdirs");
    fs::write(root.join("a/one.ts"), "export {};").expect("write file");
    fs::write(root.join("b/two.tsx"), "export {};").expect("write file");

    let text = "a/one.ts a/one.ts b/two.tsx";
    let paths = extract_candidate_file_paths(text, root, 1);
    assert_eq!(paths, vec!["a/one.ts".to_string()]);
}

#[test]
fn detect_stub_in_tool_input_flags_write_placeholder_content() {
    let args = json!({
        "path": "src/demo.ts",
        "content": "export function run(){ return \"Fallback prompt\"; } // Placeholder"
    });
    let reason = detect_stub_in_tool_input("write", &args);
    assert!(reason.is_some());
}

#[test]
fn detect_stub_in_tool_input_allows_concrete_edit_content() {
    let args = json!({
        "old_string": "return a + b;",
        "new_string": "return sanitize(a) + sanitize(b);"
    });
    let reason = detect_stub_in_tool_input("edit", &args);
    assert!(reason.is_none());
}

#[test]
fn detect_stub_in_tool_input_flags_multiedit_stub_line() {
    let args = json!({
        "edits": [
            {
                "file_path": "src/a.ts",
                "old_string": "x",
                "new_string": "throw new Error(\"Not implemented\")"
            }
        ]
    });
    let reason = detect_stub_in_tool_input("multiedit", &args);
    assert!(reason.is_some());
}

#[test]
fn detect_stub_in_tool_input_allows_html_placeholder_attribute() {
    let args = json!({
        "old_string": "<input type=\"text\" />",
        "new_string": "<input type=\"text\" placeholder=\"Search users\" />"
    });
    let reason = detect_stub_in_tool_input("edit", &args);
    assert!(reason.is_none());
}

#[test]
fn detect_stub_in_tool_input_flags_placeholder_stub_phrase() {
    let args = json!({
        "path": "src/demo.ts",
        "content": "// Placeholder implementation\nexport const run = () => null;"
    });
    let reason = detect_stub_in_tool_input("write", &args);
    assert!(reason.is_some());
}

#[test]
fn looks_like_build_execution_request_detects_fix_prompt() {
    assert!(looks_like_build_execution_request(
        "yes fix it and patch src/tool/lsp.rs"
    ));
    assert!(!looks_like_build_execution_request(
        "what does this module do?"
    ));
}

#[test]
fn codesearch_no_match_detection_matches_expected_format() {
    assert!(is_codesearch_no_match_output(
        "codesearch",
        true,
        "No matches found for pattern: foo_bar"
    ));
}

#[test]
fn codesearch_no_match_detection_matches_prefixed_output() {
    assert!(is_codesearch_no_match_output(
        "codesearch",
        true,
        "build step 1 codesearch: No matches found for pattern: foo_bar"
    ));
}

#[test]
fn codesearch_no_match_detection_ignores_non_codesearch_tools() {
    assert!(!is_codesearch_no_match_output(
        "grep",
        true,
        "No matches found for pattern: foo_bar"
    ));
}

#[test]
fn build_mode_tool_first_retry_triggers_for_deferral_reply() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("src/provider")).expect("mkdirs");
    fs::write(root.join("src/provider/gemini_web.rs"), "fn main(){}").expect("write file");

    let tools = vec![ToolDefinition {
        name: "bash".to_string(),
        description: "Run shell commands".to_string(),
        parameters: json!({"type":"object"}),
    }];
    let session_messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "fix src/provider/gemini_web.rs now".to_string(),
        }],
    }];

    let should_retry = should_force_build_tool_first_retry(
        "build",
        0,
        &tools,
        &session_messages,
        root,
        "If you want, I can patch this next.",
        false,
        2,
    );

    assert!(should_retry);
}

#[test]
fn build_mode_tool_first_retry_does_not_trigger_for_non_build_agent() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("src/provider")).expect("mkdirs");
    fs::write(root.join("src/provider/gemini_web.rs"), "fn main(){}").expect("write file");

    let tools = vec![ToolDefinition {
        name: "bash".to_string(),
        description: "Run shell commands".to_string(),
        parameters: json!({"type":"object"}),
    }];
    let session_messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "fix src/provider/gemini_web.rs now".to_string(),
        }],
    }];

    let should_retry = should_force_build_tool_first_retry(
        "plan",
        0,
        &tools,
        &session_messages,
        root,
        "If you want, I can patch this next.",
        false,
        2,
    );

    assert!(!should_retry);
}

#[test]
fn build_request_requires_tool_detects_existing_file_edit_prompt() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("src/provider")).expect("mkdirs");
    fs::write(root.join("src/provider/gemini_web.rs"), "fn main(){}").expect("write file");

    let session_messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "In build mode, make a concrete change now: in src/provider/gemini_web.rs replace \"A\" with \"B\".".to_string(),
        }],
    }];

    assert!(build_request_requires_tool(&session_messages, root));
}

#[test]
fn build_request_requires_tool_for_affirmative_followup_with_context() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::write(root.join("rspack.config.js"), "module.exports = {};").expect("write file");

    let session_messages = vec![
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "ws error in rspack.config.js".to_string(),
            }],
        },
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "Done — use this exact block in rspack.config.js".to_string(),
            }],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "yes".to_string(),
            }],
        },
    ];

    assert!(build_request_requires_tool(&session_messages, root));
}

#[test]
fn build_request_requires_tool_for_affirmative_followup_after_assistant_offer() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    let session_messages = vec![
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "the banner is missing".to_string(),
            }],
        },
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "If you want, next I can tighten the Catalyst alert variant exactly."
                    .to_string(),
            }],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "yes".to_string(),
            }],
        },
    ];

    assert!(build_request_requires_tool(&session_messages, root));
}

#[test]
fn build_request_requires_tool_false_for_plain_yes_without_context() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    let session_messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "yes".to_string(),
        }],
    }];

    assert!(!build_request_requires_tool(&session_messages, root));
}

#[test]
fn normalize_tool_call_for_execution_maps_advanced_edit_to_edit_shape() {
    let args = json!({
        "filePath": "src/lib.rs",
        "oldString": "old",
        "newString": "new"
    });
    let (name, normalized) = normalize_tool_call_for_execution("advanced_edit", &args);
    assert_eq!(name, "edit");
    assert_eq!(normalized["path"], "src/lib.rs");
    assert_eq!(normalized["old_string"], "old");
    assert_eq!(normalized["new_string"], "new");
}

#[test]
fn normalize_tool_call_for_execution_maps_multiedit_aliases() {
    let args = json!({
        "changes": [
            {
                "filePath": "src/main.rs",
                "oldString": "old",
                "newString": "new"
            }
        ]
    });
    let (name, normalized) = normalize_tool_call_for_execution("multiedit", &args);
    assert_eq!(name, "multiedit");
    assert_eq!(normalized["edits"][0]["file"], "src/main.rs");
    assert_eq!(normalized["edits"][0]["old_string"], "old");
    assert_eq!(normalized["edits"][0]["new_string"], "new");
}
