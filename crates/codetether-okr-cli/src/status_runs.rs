//! OKR status runs behavior.
use codetether_okr::OkrRepository;
use uuid::Uuid;
pub(super) async fn print(repo: &OkrRepository, uuid: Uuid) -> anyhow::Result<()> {
    // Show runs
    let runs = repo.query_runs_by_okr(uuid).await?;
    if !runs.is_empty() {
        println!("\nRuns:");
        for run in &runs {
            println!("  [{}] {} - {:?}", run.id, run.name, run.status);
            if let Some(corr) = &run.correlation_id {
                println!("    Correlation: {}", corr);
            }
        }
    }

    Ok(())
}
