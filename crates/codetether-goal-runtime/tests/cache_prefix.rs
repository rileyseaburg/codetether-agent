use chrono::Utc;
use codetether_codex_protocol::CodexProtocol;
use codetether_context_policy::{derivation::options::RequestOptions, request::from_derived};
use codetether_goal_core::{GoalRuntimeUpdate, GoalSourceKind, TaskEvent, TaskLog};
use codetether_provider_core::{ContentPart, Message, Role};

fn message(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
fn request(log: &TaskLog) -> serde_json::Value {
    let prompt = codetether_goal_runtime::compose_from_log("stable rules".into(), log);
    let history = vec![
        message(Role::User, "initial request"),
        message(Role::Assistant, "verified earlier result"),
        message(Role::User, "continue"),
    ];
    let request = from_derived(&prompt, history, "gpt-5.4", &[], RequestOptions::default());
    CodexProtocol::build_http_responses_body(&request, "gpt-5.4".into(), None, None)
}

#[test]
fn actual_goal_accounting_changes_leave_wire_cache_key_and_history_prefix_stable() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("goal.jsonl"));
    log.append_blocking(&TaskEvent::GoalSet {
        at: Utc::now(),
        goal_id: "g".into(),
        objective: "Finish the requested work".into(),
        success_criteria: vec![],
        forbidden: vec!["Do not deploy".into()],
        source_session_id: "s".into(),
        source_turn_id: "0".into(),
        source_text_hash: "hash".into(),
        source_kind: GoalSourceKind::UserProvided,
        confidence: 1.0,
    })
    .unwrap();
    let first = request(&log);
    log.append_blocking(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: "g".into(),
        objective: None,
        status: None,
        token_budget: None,
        token_delta: 750,
        elapsed_seconds: 4,
        continuation_delta: 1,
    }))
    .unwrap();
    let second = request(&log);
    assert_eq!(first["instructions"], "stable rules");
    assert_eq!(first["instructions"], second["instructions"]);
    assert_eq!(first["prompt_cache_key"], second["prompt_cache_key"]);
    let a = first["input"].as_array().unwrap();
    let b = second["input"].as_array().unwrap();
    assert_eq!(&a[..2], &b[..2]);
    assert_ne!(a[2], b[2]);
    assert_eq!(a[3], b[3]);
    assert!(b[2].to_string().contains("750 tokens"));
    assert!(b[2].to_string().contains("Do not deploy"));
    assert!(!second["instructions"].to_string().contains("750"));
}
