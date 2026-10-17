//! Application forage regression coverage.
use super::*;

#[tokio::test]
async fn swarm_success_without_file_evidence_does_not_increment_progress() {
    let dir = tempdir().expect("create tempdir");
    let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

    let mut okr = Okr::new("Autonomous Business-Aligned Execution", "Test objective");
    okr.status = OkrStatus::Active;
    let mut kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
    kr.update_progress(10.0);
    let kr_id = kr.id;
    okr.add_key_result(kr);
    let okr_id = okr.id;
    let _ = repo.create_okr(okr).await.expect("create okr");

    let item = opportunity::new(okr_id, kr_id, 0.10, 0.90);
    let args = args::new(
        "swarm".to_string(),
        Some("openai-codex/gpt-5.1-codex".to_string()),
    );
    let execution_outcome = ExecutionOutcome {
        detail: "swarm execution completed".to_string(),
        changed_files: Vec::new(),
        quality_gates_passed: true,
    };
    record_execution_success_to_okr(&repo, &item, &args, &execution_outcome, 2)
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
    assert_eq!(saved_kr.current_value, 10.0);
    assert_eq!(saved_kr.outcomes.len(), 1);
    assert!(
        saved_kr.outcomes[0]
            .evidence
            .iter()
            .any(|entry| entry == "concrete_file_evidence:false")
    );
}
