//! Model-created goal provenance comes from host evidence, never input claims.
use crate::{GoalSource, Host};
use codetether_goal_core::{
    GoalEdit, GoalSourceKind, TaskLog, TaskState, control::GoalControlError,
};
use codetether_tool_core::Tool;
use std::sync::Mutex;

static LOG: Mutex<Option<std::path::PathBuf>> = Mutex::new(None);
struct TestHost;
impl Host for TestHost {
    async fn current(_: &str) -> anyhow::Result<(TaskLog, TaskState)> {
        let log = TaskLog::at(LOG.lock().unwrap().as_ref().unwrap().clone());
        let state = TaskState::from_log(&log.read_all().await?);
        Ok((log, state))
    }
    async fn source(_: &str, objective: &str) -> anyhow::Result<Option<GoalSource>> {
        if objective == "source failure" {
            anyhow::bail!("fixture source is unavailable");
        }
        Ok((objective != "unverified").then(|| GoalSource {
            turn_id: "message:4".into(),
            text_hash: "canonical-digest".into(),
            verbatim: objective == "original request",
        }))
    }
    async fn update(_: &str, _: GoalEdit) -> Result<serde_json::Value, GoalControlError> {
        unreachable!()
    }
}

#[tokio::test]
async fn creation_records_raw_source_without_promoting_a_paraphrase_to_user_words() {
    let root = tempfile::tempdir().unwrap();
    for (i, objective, expected_kind, expected_confidence) in [
        (0, "original request", GoalSourceKind::UserProvided, 1.0),
        (
            1,
            "expanded implementation plan",
            GoalSourceKind::RawTurn,
            0.5,
        ),
        (2, "unverified", GoalSourceKind::Inferred, 0.0),
        (3, "source failure", GoalSourceKind::Inferred, 0.0),
    ] {
        let path = root.path().join(format!("{i}.jsonl"));
        *LOG.lock().unwrap() = Some(path.clone());
        let args = serde_json::from_value(serde_json::json!({
            "objective": objective, "__ct_session_id": "s",
            "source_kind": "user_provided", "confidence": 1.0, "source_text_hash": "model-spoofed",
        }))
        .unwrap();
        let result = crate::create::<TestHost>(args).await.unwrap();
        assert!(result.success);
        let output: serde_json::Value = serde_json::from_str(&result.output).unwrap();
        assert_eq!(output["goal"]["source"]["confidence"], expected_confidence);
        assert_eq!(output["goal"]["source"]["hasSourceReference"], i < 2);
        let loaded = crate::GetGoalTool::<TestHost>::new()
            .execute(serde_json::json!({"__ct_session_id": "s"}))
            .await
            .unwrap();
        let loaded: serde_json::Value = serde_json::from_str(&loaded.output).unwrap();
        assert_eq!(loaded["goal"]["source"], output["goal"]["source"]);
        let events = TaskLog::at(path).read_all().await.unwrap();
        let codetether_goal_core::TaskEvent::GoalSet {
            source_kind,
            confidence,
            source_turn_id,
            source_text_hash,
            ..
        } = &events[0]
        else {
            panic!("goal declaration")
        };
        assert_eq!(source_kind, &expected_kind);
        assert_eq!(*confidence, expected_confidence);
        assert_eq!(source_turn_id, if i >= 2 { "" } else { "message:4" });
        assert_eq!(
            source_text_hash,
            if i >= 2 { "" } else { "canonical-digest" }
        );
        let mut state = TaskState::from_log(&events);
        state.goal.as_mut().unwrap().objective = "subsequently changed objective".into();
        let changed = crate::response::with_source(&state, &events);
        let output: serde_json::Value = serde_json::from_str(&changed.output).unwrap();
        assert!(output["goal"]["source"].is_null());
    }
}
