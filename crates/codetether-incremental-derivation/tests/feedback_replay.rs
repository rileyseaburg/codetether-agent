//! Read-only incident replay: corrections must reach the final provider request.
#[path = "replay_support/feedback.rs"]
mod feedback;
mod replay_support;
use anyhow::Result;
use replay_support::load;

#[tokio::test]
#[ignore = "requires CODETETHER_REPLAY_SESSION pointing to incident session 32e98b82"]
async fn incident_32e98b82_preserves_feedback_at_continuation_boundaries() -> Result<()> {
    let path = std::env::var("CODETETHER_REPLAY_SESSION")?;
    let (mut source, model, count) = load(std::path::Path::new(&path))?;
    let original = source.messages.clone();
    let mut report = Vec::new();
    for end in [4414, 4790, 4820] {
        anyhow::ensure!(
            count >= end && source.offset < end,
            "wrong incident snapshot"
        );
        source.messages = original[..end - source.offset].to_vec();
        for budget in [16_000, 32_000, 64_000] {
            report.push(feedback::project(&source, &model, end, budget).await?);
        }
    }
    let report = serde_json::to_string_pretty(&report)?;
    if let Ok(path) = std::env::var("CODETETHER_REPLAY_REPORT") {
        std::fs::write(path, &report)?;
    }
    eprintln!("{report}");
    Ok(())
}
