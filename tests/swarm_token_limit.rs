//! Regression tests for sub-agent token-limit exhaustion (issue #218).
//!
//! These tests validate that long-running sub-agents returning
//! token-exhaustion failures produce structured reports with partial
//! progress and recovery instructions, and that truncation preserves
//! critical evidence.

use codetether_agent::provider::{ContentPart, Message, Role};
use codetether_agent::swarm::token_exhaustion::{
    RecoveryAdvice, TokenExhaustionReport, extract_evidence,
};
use codetether_agent::swarm::token_truncate::{
    DEFAULT_CONTEXT_LIMIT, estimate_message_tokens, estimate_tokens, estimate_total_tokens,
    truncate_large_tool_results, truncate_messages_to_fit, truncate_single_result,
};

// ---------------------------------------------------------------------------
// Token estimation
// ---------------------------------------------------------------------------

#[test]
fn estimate_tokens_reasonable_range() {
    let tokens = estimate_tokens("The quick brown fox jumps over the lazy dog.");
    assert!(
        (5..30).contains(&tokens),
        "expected ~13 tokens, got {tokens}"
    );
}

#[test]
fn estimate_total_tokens_multi_message() {
    let msgs = vec![
        Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: "You are helpful.".into(),
            }],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "hello".into(),
            }],
        },
    ];
    let total = estimate_total_tokens(&msgs);
    assert!(total > 0, "total tokens should be positive");
}

#[test]
fn estimate_message_tokens_with_tool_result() {
    let msg = Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "tc-1".into(),
            content: "x".repeat(10_000),
        }],
    };
    let tokens = estimate_message_tokens(&msg);
    assert!(tokens > 2000, "large tool result should be many tokens");
}

// ---------------------------------------------------------------------------
// Single-result truncation
// ---------------------------------------------------------------------------

#[test]
fn truncate_preserves_short_content() {
    let content = "short".to_string();
    let result = truncate_single_result(&content, 100);
    assert_eq!(result, content);
}

#[test]
fn truncate_adds_notice() {
    let content = "a".repeat(5000);
    let result = truncate_single_result(&content, 100);
    assert!(result.contains("TRUNCATED"));
    assert!(result.len() < 500);
}

// ---------------------------------------------------------------------------
// Message-list truncation (core truncation strategy)
// ---------------------------------------------------------------------------

/// Build a conversation large enough to trigger truncation.
fn make_large_conversation(n_pairs: usize, chars_per_msg: usize) -> Vec<Message> {
    let mut msgs = Vec::new();
    msgs.push(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: "system prompt".into(),
        }],
    });
    msgs.push(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "initial instruction".into(),
        }],
    });
    for i in 0..n_pairs {
        msgs.push(Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "x".repeat(chars_per_msg),
            }],
        });
        msgs.push(Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: format!("tc-{i}"),
                content: "y".repeat(chars_per_msg),
            }],
        });
    }
    for i in 0..4 {
        msgs.push(Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: format!("tail-{i}"),
            }],
        });
    }
    msgs
}

#[test]
fn truncate_messages_noop_on_small_history() {
    let mut msgs = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: "hi".into() }],
    }];
    let removed = truncate_messages_to_fit(&mut msgs, DEFAULT_CONTEXT_LIMIT);
    assert_eq!(removed, 0);
}

#[test]
fn truncate_messages_removes_middle_on_oversized() {
    let mut msgs = make_large_conversation(20, 50_000);
    let orig_len = msgs.len();
    let removed = truncate_messages_to_fit(&mut msgs, DEFAULT_CONTEXT_LIMIT);
    assert!(removed > 0, "should have removed middle messages");
    assert!(msgs.len() < orig_len);
    assert!(
        msgs.len() >= 5,
        "should keep at least head + summary + tail"
    );
}

#[test]
fn truncate_preserves_system_and_user_at_head() {
    let mut msgs = make_large_conversation(15, 50_000);
    let _ = truncate_messages_to_fit(&mut msgs, DEFAULT_CONTEXT_LIMIT);
    assert!(matches!(msgs[0].role, Role::System));
    assert!(matches!(msgs[1].role, Role::User));
}

#[test]
fn truncate_preserves_tail_messages() {
    let mut msgs = make_large_conversation(15, 50_000);
    let _ = truncate_messages_to_fit(&mut msgs, DEFAULT_CONTEXT_LIMIT);
    let tail_len = msgs.len();
    let last_text = match &msgs[tail_len - 1].content[0] {
        ContentPart::Text { text } => text.clone(),
        _ => String::new(),
    };
    assert!(
        last_text.starts_with("tail-"),
        "last msg should be tail-3, got: {last_text}"
    );
}

#[test]
fn truncate_inserts_summary_message() {
    let mut msgs = make_large_conversation(15, 50_000);
    let _ = truncate_messages_to_fit(&mut msgs, DEFAULT_CONTEXT_LIMIT);
    let has_summary = msgs.iter().any(|m| {
        m.content.iter().any(|p| {
            matches!(
                p,
                ContentPart::Text { text } if text.contains("Context truncated")
            )
        })
    });
    assert!(has_summary, "should insert a truncation summary");
}

// ---------------------------------------------------------------------------
// Large tool result in-place truncation
// ---------------------------------------------------------------------------

#[test]
fn truncate_large_tool_results_compresses_oversized() {
    let mut msgs = vec![Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "tc-1".into(),
            content: "z".repeat(100_000),
        }],
    }];
    let count = truncate_large_tool_results(&mut msgs, 100);
    assert_eq!(count, 1);
    match &msgs[0].content[0] {
        ContentPart::ToolResult { content, .. } => {
            assert!(content.contains("TRUNCATED"));
        }
        _ => panic!("expected ToolResult"),
    }
}

// ---------------------------------------------------------------------------
// Token-exhaustion report
// ---------------------------------------------------------------------------

#[test]
fn report_error_message_contains_subtask_id() {
    let report = TokenExhaustionReport {
        subtask_id: "st-abc".into(),
        tokens_used: 300_000,
        tokens_limit: 256_000,
        steps_completed: 50,
        changed_files: vec!["src/lib.rs".into()],
        last_tool_calls: vec!["edit".into(), "bash".into()],
        errors_encountered: vec![],
        partial_output: "Half done".into(),
        recovery_hint: "Continue from step 51".into(),
    };
    let msg = report.to_error_message();
    assert!(msg.contains("st-abc"));
    assert!(msg.contains("300000/256000"));
    assert!(msg.contains("Continue from step 51"));
}

#[test]
fn report_serializes_to_json() {
    let report = TokenExhaustionReport {
        subtask_id: "st-1".into(),
        tokens_used: 256_000,
        tokens_limit: 256_000,
        steps_completed: 10,
        changed_files: vec![],
        last_tool_calls: vec![],
        errors_encountered: vec![],
        partial_output: String::new(),
        recovery_hint: String::new(),
    };
    let json = serde_json::to_string(&report).unwrap();
    assert!(json.contains("st-1"));
    let deserialized: TokenExhaustionReport = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.subtask_id, "st-1");
}

// ---------------------------------------------------------------------------
// Recovery advice
// ---------------------------------------------------------------------------

#[test]
fn recovery_advice_resumable_when_steps_completed() {
    let report = TokenExhaustionReport {
        subtask_id: "st-r".into(),
        tokens_used: 256_000,
        tokens_limit: 256_000,
        steps_completed: 15,
        changed_files: vec!["src/main.rs".into()],
        last_tool_calls: vec!["edit".into()],
        errors_encountered: vec![],
        partial_output: "Partial".into(),
        recovery_hint: "Resume".into(),
    };
    let advice = RecoveryAdvice::from_report(&report);
    assert!(advice.can_resume);
    assert!(
        advice
            .suggested_splits
            .iter()
            .any(|s| s.contains("src/main.rs"))
    );
}

#[test]
fn recovery_advice_not_resumable_when_zero_steps() {
    let report = TokenExhaustionReport {
        subtask_id: "st-nr".into(),
        tokens_used: 256_000,
        tokens_limit: 256_000,
        steps_completed: 0,
        changed_files: vec![],
        last_tool_calls: vec![],
        errors_encountered: vec![],
        partial_output: String::new(),
        recovery_hint: String::new(),
    };
    let advice = RecoveryAdvice::from_report(&report);
    assert!(!advice.can_resume);
}

// ---------------------------------------------------------------------------
// Evidence extraction
// ---------------------------------------------------------------------------

#[test]
fn extract_evidence_captures_files_and_errors() {
    let history: Vec<(&str, &str, bool)> = vec![
        ("edit", "wrote src/main.rs ok", true),
        ("bash", "cargo test FAILED", false),
        ("edit", "wrote src/lib.rs ok", true),
    ];
    let (files, tools, errors) = extract_evidence(&history);
    assert!(files.contains(&"src/main.rs".to_string()));
    assert!(files.contains(&"src/lib.rs".to_string()));
    assert!(tools.contains(&"edit".to_string()));
    assert!(tools.contains(&"bash".to_string()));
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("FAILED"));
}

#[test]
fn extract_evidence_empty_history() {
    let history: Vec<(&str, &str, bool)> = vec![];
    let (files, tools, errors) = extract_evidence(&history);
    assert!(files.is_empty());
    assert!(tools.is_empty());
    assert!(errors.is_empty());
}

// ---------------------------------------------------------------------------
// End-to-end simulation: long-running sub-agent with token pressure
// ---------------------------------------------------------------------------

#[test]
fn simulated_long_running_agent_produces_structured_failure() {
    let mut msgs = make_large_conversation(30, 60_000);
    let estimated = estimate_total_tokens(&msgs);
    assert!(
        estimated > DEFAULT_CONTEXT_LIMIT,
        "simulated conversation should exceed limit: estimated={estimated}"
    );

    let removed = truncate_messages_to_fit(&mut msgs, DEFAULT_CONTEXT_LIMIT);
    assert!(removed > 0, "truncation should have removed messages");

    let after_truncation = estimate_total_tokens(&msgs);
    assert!(
        after_truncation < estimated,
        "after={after_truncation}, before={estimated}"
    );

    let history: Vec<(&str, &str, bool)> = vec![
        ("edit", "wrote src/main.rs", true),
        ("bash", "cargo build", true),
        ("edit", "wrote src/lib.rs", true),
        ("bash", "cargo test FAILED 2 tests", false),
    ];
    let (files, tools, errors) = extract_evidence(&history);

    let report = TokenExhaustionReport {
        subtask_id: "st-sim-001".into(),
        tokens_used: estimated,
        tokens_limit: DEFAULT_CONTEXT_LIMIT,
        steps_completed: 30,
        changed_files: files,
        last_tool_calls: tools,
        errors_encountered: errors,
        partial_output: "Simulated partial output".into(),
        recovery_hint: "Resume from the 2 failing tests".into(),
    };

    let advice = RecoveryAdvice::from_report(&report);
    assert!(advice.can_resume);
    assert!(!report.changed_files.is_empty());
    assert!(!report.errors_encountered.is_empty());

    let error_msg = report.to_error_message();
    assert!(error_msg.contains("st-sim-001"));
    assert!(error_msg.contains("Resume from the 2 failing tests"));
}

#[test]
fn simulated_truncation_preserves_tail_tool_calls() {
    let mut msgs = make_large_conversation(20, 50_000);

    let tail_start = msgs.len() - 4;
    for i in 0..4 {
        msgs[tail_start + i] = Message {
            role: if i % 2 == 0 {
                Role::Assistant
            } else {
                Role::Tool
            },
            content: vec![if i % 2 == 0 {
                ContentPart::ToolCall {
                    id: format!("tc-tail-{i}"),
                    name: format!("critical_tool_{i}"),
                    arguments: "{}".into(),
                    thought_signature: None,
                }
            } else {
                ContentPart::ToolResult {
                    tool_call_id: format!("tc-tail-{}", i - 1),
                    content: format!("critical result {i}"),
                }
            }],
        };
    }

    let _ = truncate_messages_to_fit(&mut msgs, DEFAULT_CONTEXT_LIMIT);

    let tail_tool_call = msgs.iter().any(|m| {
        m.content.iter().any(|p| {
            matches!(
                p,
                ContentPart::ToolCall { name, .. } if name.starts_with("critical_tool")
            )
        })
    });
    assert!(tail_tool_call, "tail tool calls should survive truncation");
}
