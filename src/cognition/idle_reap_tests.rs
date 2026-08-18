//! Tests for idle-TTL reaping behavior.

use chrono::{Duration as ChronoDuration, Utc};
use std::time::Duration;

use super::tests_request::create;
use super::tests_support::runtime_with_policy;
use super::{CognitionRuntime, PersonaPolicy, PersonaStatus};

/// A policy that treats any persona as immediately idle.
fn immediate_idle(token_budget: u32) -> PersonaPolicy {
    PersonaPolicy {
        max_spawn_depth: 2,
        max_branching_factor: 2,
        token_budget_per_minute: token_budget,
        compute_ms_per_minute: if token_budget == 0 { 0 } else { 10_000 },
        idle_ttl_secs: 0,
        share_memory: false,
        allowed_tools: Vec::new(),
    }
}

/// Push `last_progress_at` into the past to trigger the idle check.
async fn backdate_progress(runtime: &CognitionRuntime, persona_id: &str) {
    let mut personas = runtime.personas.write().await;
    if let Some(persona) = personas.get_mut(persona_id) {
        persona.last_progress_at = Utc::now() - ChronoDuration::seconds(10);
    }
}

/// Create `id`, backdate its progress, and run one tick.
async fn run_idle_cycle(runtime: &CognitionRuntime, id: &str, role: &str) {
    runtime
        .create_persona(create(id, role, "work"))
        .await
        .expect("should create persona");
    backdate_progress(runtime, id).await;
    runtime.start(None).await.expect("should start");
    tokio::time::sleep(Duration::from_millis(100)).await;
    runtime.stop(None).await.expect("should stop");
}

#[tokio::test]
async fn idle_persona_is_reaped() {
    let runtime = runtime_with_policy(immediate_idle(20_000));
    run_idle_cycle(&runtime, "idle-test", "idler").await;
    let persona = runtime.get_persona("idle-test").await.unwrap();
    assert_eq!(persona.status, PersonaStatus::Reaped);
}

#[tokio::test]
async fn budget_paused_persona_not_reaped_for_idle() {
    let runtime = runtime_with_policy(immediate_idle(0));
    run_idle_cycle(&runtime, "paused-test", "pauser").await;
    let persona = runtime.get_persona("paused-test").await.unwrap();
    // Budget-paused personas are throttled, not stalled, so they survive.
    assert_eq!(persona.status, PersonaStatus::Active);
    assert!(persona.budget_paused);
}
