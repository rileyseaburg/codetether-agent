//! OKR create command.
use codetether_cli_options::OkrArgs;
use codetether_okr::{KeyResult, Okr, OkrRepository};
pub(super) async fn execute(args: OkrArgs, repo: &OkrRepository) -> anyhow::Result<()> {
    let title = args
        .title
        .ok_or_else(|| anyhow::anyhow!("--title required"))?;
    let description = args.description.unwrap_or_default();

    let mut okr = Okr::new(title, description);

    // Add key result if target specified
    if let Some(target) = args.target {
        let kr = KeyResult::new(okr.id, "Key Result 1", target, args.unit);
        okr.add_key_result(kr);
    }

    let created = repo.create_okr(okr).await?;
    println!("Created OKR: {} ({})", created.title, created.id);
    Ok(())
}
