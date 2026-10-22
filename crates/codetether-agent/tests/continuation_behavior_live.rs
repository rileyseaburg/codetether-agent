//! Opt-in live model capture. Tool calls are recorded and NEVER executed.
#[path = "continuation_live/capture.rs"]
mod capture;
#[path = "continuation_live/feedback.rs"]
mod feedback;
#[path = "continuation_live/progress_replay.rs"]
mod progress_replay;
#[path = "continuation_live/provider.rs"]
mod provider;
#[path = "continuation_live/refresh.rs"]
mod refresh;
#[path = "../../codetether-incremental-derivation/tests/replay_support/mod.rs"]
#[allow(dead_code)] // The shared loader also provides a context-only request fixture.
mod replay_support;
#[path = "continuation_live/request.rs"]
mod request;
use anyhow::Result;

#[tokio::test]
#[ignore = "calls the incident model; requires CODETETHER_REPLAY_SESSION and CODETETHER_REPLAY_REPORT"]
async fn capture_incident_continuation_without_executing_tools() -> Result<()> {
    let path = std::env::var("CODETETHER_REPLAY_SESSION")?;
    let report_path = std::env::var("CODETETHER_REPLAY_REPORT")?;
    let (mut source, model, count) = replay_support::load(std::path::Path::new(&path))?;
    anyhow::ensure!(count >= 4790 && source.offset < 4414, "wrong snapshot");
    let original = source.messages.clone();
    let pages = source.pages.clone();
    let provider = provider::load().await?;
    let mut captures = Vec::new();
    for end in [4414, 4790] {
        source.messages = original[..end - source.offset].to_vec();
        source.pages = pages[..end - source.offset].to_vec();
        if std::env::var_os("CODETETHER_REPLAY_REFRESH_SUMMARIES").is_some() {
            refresh::recent(&mut source, provider.clone(), &model, &report_path).await?;
        }
        feedback::apply(&mut source, end)?;
        let request = request::build(&source, &model).await?;
        let response = tokio::time::timeout(
            std::time::Duration::from_secs(180),
            provider.complete(request),
        )
        .await??;
        let capture = capture::record(end, &model, response);
        captures.push(capture);
        std::fs::write(&report_path, serde_json::to_string_pretty(&captures)?)?;
    }
    capture::finish(&captures, &report_path)
}
