//! run for codebase indexing.

use super::*;

pub async fn run(args: crate::cli::IndexArgs) -> Result<()> {
    let root = args
        .path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let root = root.canonicalize().unwrap_or_else(|_| root.clone());

    let options = options(&args);

    let index = build_index(&root, &options).await?;
    let output_path = args.output.unwrap_or_else(|| default_index_path(&root));

    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let encoded = serde_json::to_string_pretty(&index)?;
    tokio::fs::write(&output_path, encoded).await?;

    print_summary(&index, &output_path, args.json)?;
    Ok(())
}
