//! Static Git metrics for a Kubernetes branch-observation probe.

use super::kubernetes_executor::RemoteBranchProbe;
use std::process::{Command, Output};

pub(super) fn snapshot(subtask_id: &str) -> RemoteBranchProbe {
    RemoteBranchProbe {
        subtask_id: subtask_id.to_string(),
        compile_ok: git(&["diff", "--check"]).is_some_and(|output| output.status.success()),
        changed_files: lines(&["diff", "--name-only"]),
        changed_lines: changed_lines(),
    }
}

fn changed_lines() -> u32 {
    lines(&["diff", "--numstat"])
        .iter()
        .map(|line| {
            line.split('\t')
                .take(2)
                .filter_map(|part| part.parse::<u32>().ok())
                .sum::<u32>()
        })
        .sum()
}

fn lines(args: &[&str]) -> Vec<String> {
    let Some(output) = git(args).filter(|output| output.status.success()) else {
        return Vec::new();
    };
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect()
}

fn git(args: &[&str]) -> Option<Output> {
    Command::new("git").args(args).output().ok()
}
