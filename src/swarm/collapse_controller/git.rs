use super::*;

pub(super) fn collect_changed_files(worktree_path: &PathBuf) -> Result<HashSet<String>> {
    let output = Command::new("git")
        .args(["diff", "--name-only"])
        .current_dir(worktree_path)
        .output()
        .with_context(|| {
            format!(
                "Failed to collect changed files in {}",
                worktree_path.display()
            )
        })?;
    if !output.status.success() {
        return Ok(HashSet::new());
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect())
}

pub(super) fn collect_changed_lines(worktree_path: &PathBuf) -> Result<u32> {
    let output = Command::new("git")
        .args(["diff", "--numstat"])
        .current_dir(worktree_path)
        .output()
        .with_context(|| {
            format!(
                "Failed to collect changed lines in {}",
                worktree_path.display()
            )
        })?;
    if !output.status.success() {
        return Ok(0);
    }
    let mut total = 0u32;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            continue;
        }
        total += parts[0].parse::<u32>().unwrap_or(0);
        total += parts[1].parse::<u32>().unwrap_or(0);
    }
    Ok(total)
}
