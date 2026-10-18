//! OKR report okr runs behavior.
use codetether_okr::OkrRepository;
use uuid::Uuid;
pub(super) async fn print(repo: &OkrRepository, uuid: Uuid) -> anyhow::Result<()> {
    // Show runs for this OKR
    let runs = repo.query_runs_by_okr(uuid).await?;
    if !runs.is_empty() {
        println!("\nExecution Runs:");
        for run in &runs {
            println!("\n  [{}] {}", run.id, run.name);
            println!("  Status: {:?}", run.status);
            if let Some(corr) = &run.correlation_id {
                println!("  Correlation ID: {}", corr);
            }
            if let Some(session) = &run.session_id {
                println!("  Session: {}", session);
            }
            if let Some(checkpoint) = &run.relay_checkpoint_id {
                println!("  Relay Checkpoint: {}", checkpoint);
            }
            if !run.kr_progress.is_empty() {
                println!("  KR Progress:");
                for (kr_id, progress) in &run.kr_progress {
                    println!("    {}: {:.1}%", kr_id, progress * 100.0);
                }
            }
        }
    }

    Ok(())
}
