//! tree visit for filesystem inspection.
use super::*;

/// Helper function to build tree recursively
pub(super) async fn build_tree(
    path: &Path,
    prefix: &str,
    depth: usize,
    max_depth: usize,
    show_hidden: bool,
    show_size: bool,
    respect_gitignore: bool,
    output: &mut Vec<String>,
    file_count: &mut usize,
    dir_count: &mut usize,
) -> Result<()> {
    if depth >= max_depth {
        return Ok(());
    }

    let mut entries =
        tree_collect::collect(path, show_hidden, show_size, respect_gitignore).await?;

    // Sort entries: directories first, then files, alphabetically
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    let total = entries.len();
    for (idx, entry) in entries.iter().enumerate() {
        let is_last = idx == total - 1;
        output.push(tree_line::render(
            entry, prefix, is_last, show_size, file_count, dir_count,
        ));

        // Recurse into directories
        if entry.is_dir {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            Box::pin(build_tree(
                &entry.path,
                &new_prefix,
                depth + 1,
                max_depth,
                show_hidden,
                show_size,
                respect_gitignore,
                output,
                file_count,
                dir_count,
            ))
            .await?;
        }
    }

    Ok(())
}
