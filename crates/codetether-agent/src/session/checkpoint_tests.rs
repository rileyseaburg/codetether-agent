use super::{RunCheckpoint, Session, auto_resume_prompt};
use tempfile::TempDir;

#[test]
fn auto_resume_prompt_preserves_checkpoint_fields() {
    let mut cp = RunCheckpoint::from_session_messages("apply to one job", 80, "s1", None, 42, &[]);
    cp.current_browser_url = Some("https://workday.example/job".into());
    cp.completed_actions
        .push("opened external Workday flow".into());
    cp.blockers.push("hit max-step budget".into());
    cp.next_intended_action = "submit remaining required fields".into();
    let prompt = auto_resume_prompt(&cp);
    assert!(prompt.contains("apply to one job"));
    assert!(prompt.contains("https://workday.example/job"));
    assert!(prompt.contains("opened external Workday flow"));
    assert!(prompt.contains("submit remaining required fields"));
    assert!(prompt.contains("Do not ask the user to reconstruct context"));
}

#[tokio::test]
async fn checkpoint_persists_in_metadata_and_sidecar() {
    let temp = TempDir::new().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    let mut session = Session::new().await.unwrap();
    let cp =
        RunCheckpoint::from_session_messages("finish browser task", 3, &session.id, None, 7, &[]);
    let path = session.save_run_checkpoint(cp.clone()).await.unwrap();
    assert!(path.exists());
    let loaded = Session::load(&session.id).await.unwrap();
    assert_eq!(loaded.metadata.run_checkpoint.as_ref(), Some(&cp));
    assert_eq!(loaded.load_run_checkpoint().await.unwrap(), Some(cp));
}
