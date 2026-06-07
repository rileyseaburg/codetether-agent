use codetether_agent::cli::CleanupArgs;

pub fn done(args: CleanupArgs, count: usize) {
    if args.json {
        println!(
            "{}",
            serde_json::json!({ "cleaned": count, "success": true })
        );
    } else if count > 0 {
        println!("Cleaned up {count} orphaned {}.", scope(args));
    } else {
        println!("No orphaned worktrees or branches found.");
    }
}

pub fn artifacts(args: CleanupArgs, count: usize) {
    if args.json {
        println!(
            "{}",
            serde_json::json!({ "cleaned_artifact_dirs": count, "success": true })
        );
    } else if count > 0 {
        println!("Cleaned {count} worktree target directories.");
    } else {
        println!("No worktree target directories found.");
    }
}

fn scope(args: CleanupArgs) -> &'static str {
    if args.worktrees_only {
        "worktree(s)"
    } else {
        "worktree(s)/branch(es)"
    }
}
