//! OKR runs command.
use codetether_cli_options::OkrArgs;
use codetether_okr::{Okr, OkrRepository, OkrRun};
pub(super) async fn execute(args: OkrArgs, repo: &OkrRepository) -> anyhow::Result<()> {
    let okrs = repo.list_okrs().await?;
    let runs = repo.list_runs().await?;

    if args.json {
        #[derive(serde::Serialize)]
        struct RunsOutput {
            okrs: Vec<Okr>,
            runs: Vec<OkrRun>,
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&RunsOutput { okrs, runs })?
        );
    } else {
        println!("\n=== OKR Runs ===\n");
        for run in &runs {
            let okr_title = okrs
                .iter()
                .find(|o| o.id == run.okr_id)
                .map(|o| o.title.as_str())
                .unwrap_or("Unknown");

            println!("[{}] {} -> {}", run.id, okr_title, run.name);
            println!("  Status: {:?}", run.status);
            if let Some(corr) = &run.correlation_id {
                println!("  Correlation: {}", corr);
            }
            if let Some(session) = &run.session_id {
                println!("  Session: {}", session);
            }
            println!();
        }
    }
    Ok(())
}
