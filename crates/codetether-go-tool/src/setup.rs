//! Create, approve and persist the initial objective and run.

use crate::template::create_default_okr;
use codetether_okr::{Okr, OkrRepository, OkrRun};
use uuid::Uuid;
pub(crate) async fn create(task: &str) -> (Uuid, Okr, OkrRun, String, String) {
    let okr_id = Uuid::new_v4();
    let okr = create_default_okr(okr_id, task);
    let mut run = OkrRun::new(
        okr_id,
        format!("Go {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
    );
    let _ = run.submit_for_approval();
    run.record_decision(codetether_okr::ApprovalDecision::approve(
        run.id,
        "Auto-approved via MCP go tool",
    ));

    let run_id = run.id;
    let prd_filename = format!("prd_{}.json", run_id.to_string().replace('-', "_"));
    let progress_filename = format!("progress_{}.txt", run_id.to_string().replace('-', "_"));

    // Persist OKR before execution
    if let Ok(repo) = OkrRepository::from_config().await {
        let _ = repo.create_okr(okr.clone()).await;
        let _ = repo.create_run(run.clone()).await;
    }

    (okr_id, okr, run, prd_filename, progress_filename)
}
