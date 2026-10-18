//! OKR report okr behavior.
use codetether_okr::{Okr, OkrRepository, OkrRun};
use uuid::Uuid;
pub(super) async fn execute(
    json: bool,
    evidence: bool,
    repo: &OkrRepository,
    uuid: Uuid,
    okr: Okr,
) -> anyhow::Result<()> {
    // Report on an OKR
    if json {
        #[derive(serde::Serialize)]
        struct OkrReport {
            okr: Okr,
            runs: Vec<OkrRun>,
            total_progress: f64,
        }
        let runs = repo.query_runs_by_okr(uuid).await?;
        let report = OkrReport {
            okr: okr.clone(),
            runs,
            total_progress: okr.progress() * 100.0,
        };
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        super::report_okr_render::print(&okr);
        super::report_okr_keys::print(&okr, evidence);
        super::report_okr_runs::print(repo, uuid).await?;
    }
    Ok(())
}
