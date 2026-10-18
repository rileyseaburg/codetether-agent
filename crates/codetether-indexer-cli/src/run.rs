//! run for codebase indexing.

use super::*;

/// Build an index, persist it to the selected file and print its summary.
///
/// # Arguments
/// * `args` - CLI options for scanning, embeddings and output.
/// * `resolver` - Application provider lookup used for remote embeddings.
/// # Returns
/// Unit after writing the index and printing its summary.
/// # Errors
/// Returns index-building, output-directory, file-write or serialization errors.
/// # Examples
/// ```no_run
/// # async fn example(resolver: &dyn codetether_indexer_cli::EmbeddingResolver) -> anyhow::Result<()> {
/// use clap::Parser;
/// let args = codetether_indexer_cli::IndexArgs::parse_from(["index", "--path", "."]);
/// codetether_indexer_cli::run(args, resolver).await
/// # }
/// ```
pub async fn run(args: IndexArgs, resolver: &dyn EmbeddingResolver) -> Result<()> {
    let root = args
        .path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let root = root.canonicalize().unwrap_or_else(|_| root.clone());

    let options = options(&args);

    let index = build_index_with_resolver(&root, &options, resolver).await?;
    let output_path = args.output.unwrap_or_else(|| default_index_path(&root));

    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let encoded = serde_json::to_string_pretty(&index)?;
    tokio::fs::write(&output_path, encoded).await?;

    print_summary(&index, &output_path, args.json)?;
    Ok(())
}
