//! Application forage regression coverage.
use super::*;

#[tokio::test]
async fn record_execution_success_updates_kr_progress_and_evidence() {
    let dir = tempdir().expect("create tempdir");
    let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

    let mut okr = Okr::new("Autonomous Business-Aligned Execution", "Test objective");
    okr.status = OkrStatus::Active;
    let mut kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
    kr.update_progress(0.0);
    let kr_id = kr.id;
    okr.add_key_result(kr);
    let okr_id = okr.id;
    let _ = repo.create_okr(okr).await.expect("create okr");

    let item = opportunity::new(okr_id, kr_id, 0.0, 1.0);
    let args = args::new(
        "run".to_string(),
        Some("openai-codex/gpt-5.1-codex".to_string()),
    );

    let execution_outcome = ExecutionOutcome {
        detail: "run execution completed".to_string(),
        changed_files: vec!["src/forage/mod.rs".to_string()],
        quality_gates_passed: true,
    };
    record_execution_success_to_okr(&repo, &item, &args, &execution_outcome, 1)
        .await
        .expect("record success");

    let saved = repo
        .get_okr(okr_id)
        .await
        .expect("read okr")
        .expect("okr exists");
    let saved_kr = saved
        .key_results
        .into_iter()
        .find(|k| k.id == kr_id)
        .expect("kr exists");
    assert!(saved_kr.current_value > 0.0);
    assert_eq!(saved_kr.outcomes.len(), 1);
    assert!(
        saved_kr.outcomes[0]
            .evidence
            .iter()
            .any(|entry| entry.starts_with("engine:run"))
    );
}
