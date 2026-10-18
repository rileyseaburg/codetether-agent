//! OKR report run behavior.
use codetether_okr::{Okr, OkrRepository, OkrRun};
pub(super) async fn execute(json: bool, repo: &OkrRepository, run: OkrRun) -> anyhow::Result<()> {
    // Report on a specific run
    if json {
        // Get the associated OKR
        let okr = repo.get_okr(run.okr_id).await?;
        #[derive(serde::Serialize)]
        struct RunReport {
            run: OkrRun,
            okr: Option<Okr>,
        }
        let report = RunReport {
            run: run.clone(),
            okr,
        };
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        super::report_run_text::print(&run);
        // Show the parent OKR
        if let Some(okr) = repo.get_okr(run.okr_id).await? {
            println!("\nParent OKR: {}", okr.title);
            println!("OKR Status: {:?}", okr.status);
        }
    }
    Ok(())
}
