//! OKR stats command.
use codetether_cli_options::OkrArgs;
use codetether_okr::OkrRepository;
pub(super) async fn execute(args: OkrArgs, repo: &OkrRepository) -> anyhow::Result<()> {
    let stats = repo.stats().await?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        println!("\n=== OKR Statistics ===\n");
        println!("Total OKRs: {}", stats.total_okrs);
        println!("Total Runs: {}", stats.total_runs);

        println!("\nOKR Status:");
        for (status, count) in &stats.okr_status_counts {
            println!("  {:?}: {}", status, count);
        }

        println!("\nRun Status:");
        for (status, count) in &stats.run_status_counts {
            println!("  {:?}: {}", status, count);
        }
    }
    Ok(())
}
