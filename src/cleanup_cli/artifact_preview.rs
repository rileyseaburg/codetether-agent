use codetether_agent::cli::CleanupArgs;
use std::path::PathBuf;

pub fn print(args: CleanupArgs, dirs: &[PathBuf]) -> anyhow::Result<()> {
    if args.json {
        let dirs = dirs
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "dry_run": true,
                "artifacts": dirs,
            }))?
        );
        return Ok(());
    }
    println!("# Artifact Cleanup Preview (Dry Run)\n");
    if dirs.is_empty() {
        println!("No worktree target directories found.");
    } else {
        println!("Found {} target directories to clean:\n", dirs.len());
        for dir in dirs {
            println!("- {}", dir.display());
        }
        println!("\nRun without --dry-run to delete build artifacts only.");
    }
    Ok(())
}
