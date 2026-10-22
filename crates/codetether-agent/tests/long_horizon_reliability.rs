//! Offline state-fidelity evaluation: actual storage, derivation and request encoding.
#[path = "long_horizon/checkpoints.rs"]
mod checkpoints;
#[path = "long_horizon/fixtures.rs"]
mod fixtures;
#[path = "long_horizon/provider.rs"]
mod provider;
#[path = "long_horizon/requests.rs"]
mod requests;
#[path = "long_horizon/summaries.rs"]
mod summaries;
use codetether_context_pin::{PinSource, apply_to_session};
use codetether_goal_core::{GoalRuntimeUpdate, GoalStatus, TaskEvent, TaskLog};
use codetether_provider_core::Role;
use codetether_run_checkpoint::ActionStatus;
use serde_json::json;
use std::sync::{Arc, atomic::Ordering};

#[tokio::test]
async fn repeated_compaction_restart_correction_failure_and_model_switch() -> anyhow::Result<()> {
    let root = tempfile::tempdir()?;
    checkpoints::DIRECTORY.set(root.path().into()).unwrap();
    let log = TaskLog::at(root.path().join("goal.jsonl"));
    let session = fixtures::session().await?;
    let _location = fixtures::LocationGuard(session.id.clone());
    let path = root.path().join(format!("{}.json", session.id));
    fixtures::journal(&log, &session)?;
    codetether_session_store::save(&session, &path).await?;
    let mut session = codetether_session_store::load_sync::<codetether_agent::session::Session>(
        &path,
        1_000,
        |_| Ok(()),
    )?
    .session;
    let hash = session.constraint_source(0).unwrap().text_hash;
    let corrected=apply_to_session(&mut session,json!({"action":"correct","turn_index":0,"replacement_index":1,"source_text":"Use red.","source_hash":hash})).await?;
    assert!(corrected.success, "{}", corrected.output);
    assert_eq!(
        codetether_session_store::lifecycle::session_path(&session.id)?,
        path,
        "prepared-context lookups must stay in the temporary session store"
    );
    summaries::seed(&mut session).await?;
    let provider = Arc::new(provider::RejectingProvider::default());
    let mut phases = vec![];
    for (phase, model, force, expected) in [
        (0, "gpt-5.4", Some(4), "Use blue."),
        (1, "claude-sonnet-4-5", None, "Use blue."),
        (2, "gpt-4o", Some(8), "Use green."),
    ] {
        session = codetether_session_store::load_sync(&path, 1_000, |_| Ok(()))?.session;
        summaries::verify(&session, phase);
        assert_eq!(session.metadata.progress.observations, 3);
        assert_eq!(
            session
                .metadata
                .progress
                .blocked_edit("edit", &fixtures::edit_input())
                .is_some(),
            phase < 2
        );
        let continuation = codetether_goal_runtime::next_message::<fixtures::FileJournal>(
            log.path().to_str().unwrap(),
        )
        .await?
        .unwrap();
        session.metadata.progress.human_input(&continuation);
        session.add_message(continuation);
        session.add_message(fixtures::text(
            Role::Assistant,
            "Uncertain why the target differs. Next: inspect the source before retrying.",
        ));
        let projected = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            requests::verify(&session, &log, model, force, provider.clone(), expected),
        )
        .await??;
        let checkpoint = session
            .build_run_checkpoint("repair safely", 2, root.path())
            .await?;
        assert!(checkpoint.completed_actions.is_empty());
        assert_eq!(
            checkpoint
                .actions
                .iter()
                .filter(|action| action.status == ActionStatus::Failed)
                .count(),
            3
        );
        assert_eq!(
            checkpoint
                .actions
                .iter()
                .filter(|action| action.status == ActionStatus::Attempted)
                .count(),
            1
        );
        assert!(
            checkpoint
                .user_context
                .iter()
                .any(|source| source.source_turn == 0 && source.text == fixtures::ORIGINAL)
        );
        assert!(
            checkpoint
                .assistant_context
                .as_deref()
                .unwrap()
                .contains("Uncertain")
        );
        assert!(checkpoint.next_intended_action.contains("inspect"));
        codetether_run_checkpoint::storage::save(
            &mut checkpoints::Native(&mut session),
            checkpoint.clone(),
        )
        .await?;
        let reloaded = codetether_session_store::load_sync::<codetether_agent::session::Session>(
            &path,
            1_000,
            |_| Ok(()),
        )?
        .session;
        assert_eq!(
            reloaded.load_run_checkpoint().await?.unwrap().actions,
            checkpoint.actions
        );
        phases.push(json!({"phase":phase,"model":model,"canonical_messages":session.message_count(),"request_messages":projected,"fixture_tokens":codetether_goal_runtime::state_cache::load(&log)?.goal.unwrap().tokens_used}));
        if phase == 0 {
            summaries::invalidate_one(&mut session);
            session.save().await?;
        }
        if phase == 1 {
            let source = session.constraint_source(1).unwrap();
            let next = session.message_count();
            let correction = fixtures::text(Role::User, "Use green.");
            session.metadata.progress.human_input(&correction);
            session.add_message(correction);
            let result=apply_to_session(&mut session,json!({"action":"correct","turn_index":1,"replacement_index":next,"source_text":"Use blue.","source_hash":source.text_hash})).await?;
            assert!(result.success, "{}", result.output);
        }
    }
    let before = session.metadata.progress.novel_observations;
    session.add_message(fixtures::call("fresh", "read", json!({"path":"target"})));
    session.add_message(fixtures::reply("fresh", true, "new source evidence"));
    assert_eq!(session.metadata.progress.novel_observations, before + 1);
    codetether_run_checkpoint::storage::clear(&mut checkpoints::Native(&mut session)).await?;
    let reloaded = codetether_session_store::load_sync::<codetether_agent::session::Session>(
        &path,
        1_000,
        |_| Ok(()),
    )?
    .session;
    assert!(reloaded.load_run_checkpoint().await?.is_none());
    assert_eq!(reloaded.metadata.constraint_corrections.len(), 2);
    log.append_blocking(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: chrono::Utc::now(),
        goal_id: "evaluation".into(),
        objective: None,
        status: Some(GoalStatus::Paused),
        token_budget: None,
        token_delta: 0,
        elapsed_seconds: 0,
        continuation_delta: 0,
    }))?;
    assert!(
        codetether_goal_runtime::next_message::<fixtures::FileJournal>(
            log.path().to_str().unwrap()
        )
        .await?
        .is_none()
    );
    let blocked_attempts = provider.0.load(Ordering::SeqCst);
    assert!(
        blocked_attempts > 0,
        "provider-outage fallback must be exercised"
    );
    if let Ok(report) = std::env::var("CODETETHER_EVAL_REPORT") {
        std::fs::write(
            report,
            serde_json::to_vec_pretty(
                &json!({"kind":"deterministic-state-evaluation","phases":phases,"blocked_provider_attempts":blocked_attempts,"live_provider_requests":0,"live_cache_hit_rate":null,"live_latency":null,"live_cost_savings":null}),
            )?,
        )?;
    }
    Ok(())
}
