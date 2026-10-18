//! OKR export command.
use codetether_cli_options::OkrArgs;
use codetether_okr::{Okr, OkrRepository, OkrRun};
pub(super) async fn execute(args: OkrArgs, repo: &OkrRepository) -> anyhow::Result<()> {
    let okrs = repo.list_okrs().await?;
    let runs = repo.list_runs().await?;

    #[derive(serde::Serialize)]
    struct ExportData {
        okrs: Vec<Okr>,
        runs: Vec<OkrRun>,
        exported_at: chrono::DateTime<chrono::Utc>,
    }

    let export = ExportData {
        okrs,
        runs,
        exported_at: chrono::Utc::now(),
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&export)?);
    } else {
        println!(
            "Exported {} OKRs and {} runs",
            export.okrs.len(),
            export.runs.len()
        );
    }
    Ok(())
}
