//! execute for OKR tool dispatch.
use super::*;
pub(super) async fn execute(params: Value) -> Result<ToolResult> {
    let p: Params = serde_json::from_value(params).context("Invalid OKR tool params")?;
    let repo = get_repo().await?;

    match p.action.as_str() {
        "create_okr" => create_okr::run(repo, p).await,
        "get_okr" => get_okr::run(repo, p).await,
        "update_okr" => update_okr::run(repo, p).await,
        "delete_okr" => delete_okr::run(repo, p).await,
        "list_okrs" => list_okrs::run(repo, p).await,
        "query_okrs" => query_okrs::run(repo, p).await,
        "create_run" => create_run::run(repo, p).await,
        "get_run" => get_run::run(repo, p).await,
        "update_run" => update_run::run(repo, p).await,
        "delete_run" => delete_run::run(repo, p).await,
        "list_runs" => list_runs::run(repo, p).await,
        "query_runs" => query_runs::run(repo, p).await,
        "stats" => stats::run(repo, p).await,
        _ => Ok(ToolResult::error(format!(
            "Unknown action: {}. Use: create_okr, get_okr, update_okr, delete_okr, \
                 list_okrs, query_okrs, create_run, get_run, update_run, delete_run, \
                 list_runs, query_runs, stats",
            p.action
        ))),
    }
}
