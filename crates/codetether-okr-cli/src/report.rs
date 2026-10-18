//! OKR report command.
use codetether_cli_options::OkrArgs;
use codetether_okr::OkrRepository;
use uuid::Uuid;
pub(super) async fn execute(args: OkrArgs, repo: &OkrRepository) -> anyhow::Result<()> {
    let id = args
        .id
        .ok_or_else(|| anyhow::anyhow!("--id required for report"))?;
    let uuid = Uuid::parse_str(&id)?;

    // Try to find as OKR first, then as run
    let okr = repo.get_okr(uuid).await?;
    let run = repo.get_run(uuid).await?;

    if let Some(okr) = okr {
        super::report_okr::execute(args.json, args.evidence, repo, uuid, okr).await
    } else if let Some(run) = run {
        super::report_run::execute(args.json, repo, run).await
    } else {
        anyhow::bail!("OKR or Run not found: {}", id)
    }
}
