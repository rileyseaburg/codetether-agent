//! OKR status command.
use codetether_cli_options::OkrArgs;
use codetether_okr::OkrRepository;
use uuid::Uuid;
pub(super) async fn execute(args: OkrArgs, repo: &OkrRepository) -> anyhow::Result<()> {
    let id = args
        .id
        .ok_or_else(|| anyhow::anyhow!("--id required for status"))?;
    let uuid = Uuid::parse_str(&id)?;

    let okr = repo
        .get_okr(uuid)
        .await?
        .ok_or_else(|| anyhow::anyhow!("OKR not found"))?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&okr)?);
    } else {
        super::status_render::print(&okr);
        super::status_runs::print(repo, uuid).await?;
    }
    Ok(())
}
