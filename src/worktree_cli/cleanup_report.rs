use codetether_agent::worktree::maintenance::{WorktreeCleanupReport, WorktreeCleanupState};

pub(super) fn print(report: &WorktreeCleanupReport, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(report)?);
        return Ok(());
    }
    for entry in &report.entries {
        let branch = entry.branch.as_deref().unwrap_or("(detached)");
        println!(
            "{}\t{}\t{}",
            entry.state.as_str(),
            branch,
            entry.path.display()
        );
        if let Some(error) = &entry.error {
            println!("  error: {error}");
        }
    }
    print_summary(report);
    Ok(())
}

fn print_summary(report: &WorktreeCleanupReport) {
    let ready =
        count(report, WorktreeCleanupState::Ready) + count(report, WorktreeCleanupState::Prunable);
    let removed = count(report, WorktreeCleanupState::Removed);
    if report.applied {
        println!("Removed {removed} worktree(s); branches were preserved.");
    } else {
        println!("Preview: {ready} worktree(s) are safe to remove; no changes made.");
        if ready > 0 {
            println!("Rerun with --apply to perform this cleanup.");
        }
    }
}

fn count(report: &WorktreeCleanupReport, state: WorktreeCleanupState) -> usize {
    report
        .entries
        .iter()
        .filter(|entry| entry.state == state)
        .count()
}
