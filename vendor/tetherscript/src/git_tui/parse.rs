//! Parse git porcelain output into panel data.

use super::model::{GitEntry, GitEntryKind, GitPanel};

/// Parse `git status --short --branch` output.
pub fn parse_status(text: &str) -> GitPanel {
    let mut branch = String::from("unknown");
    let mut entries = Vec::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            branch = rest.to_string();
        } else if line.len() >= 4 {
            entries.push(entry(line));
        }
    }
    GitPanel { branch, entries }
}

fn entry(line: &str) -> GitEntry {
    let code = line[..2].to_string();
    let path = line[3..].to_string();
    let kind = if code.contains('U') || code == "AA" || code == "DD" {
        GitEntryKind::Conflicted
    } else if code == "??" {
        GitEntryKind::Untracked
    } else if code.as_bytes()[0] != b' ' {
        GitEntryKind::Staged
    } else {
        GitEntryKind::Unstaged
    };
    GitEntry { code, path, kind }
}
