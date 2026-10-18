use super::{PromptRunResult, notice};
use crate::session::Session;
use crate::tui::app::session_runtime::SessionNotice;

#[tokio::test]
async fn application_error_preserves_its_cause_chain() {
    let session = Session::new().await.expect("session");
    let error = anyhow::anyhow!("protocol status 132").context("Gemini Web completion failed");
    let result: PromptRunResult = Ok(Err(error));
    let SessionNotice::Failed { error, .. } = notice(result, session) else {
        panic!("expected failure notice");
    };
    assert!(error.contains("Gemini Web completion failed"));
    assert!(error.contains("protocol status 132"));
}
