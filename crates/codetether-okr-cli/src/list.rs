//! OKR list command.
use codetether_cli_options::OkrArgs;
use codetether_okr::{OkrRepository, OkrStatus};
pub(super) async fn execute(args: OkrArgs, repo: &OkrRepository) -> anyhow::Result<()> {
    let okrs = if let Some(status) = &args.status {
        let status = match status.as_str() {
            "draft" => OkrStatus::Draft,
            "active" => OkrStatus::Active,
            "completed" => OkrStatus::Completed,
            "cancelled" => OkrStatus::Cancelled,
            "on_hold" => OkrStatus::OnHold,
            _ => anyhow::bail!("Invalid status: {}", status),
        };
        repo.query_okrs_by_status(status).await?
    } else if let Some(owner) = &args.owner {
        repo.query_okrs_by_owner(owner).await?
    } else {
        repo.list_okrs().await?
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&okrs)?);
    } else {
        super::list_render::print(&okrs, args.evidence);
    }
    Ok(())
}
