//! Runtime-mode and available-tool prompt sections.

pub(super) fn workspace_mode(shared: bool, read_only: bool, writable: bool) -> &'static str {
    if shared {
        "SHARED WRITABLE WORKSPACE: Every worker can edit files in this same directory. Make the changes required by your assignment; an inspection-only assignment does not require unnecessary edits. Work on your assigned files, preserve other workers' changes, and coordinate overlapping edits through swarm_share. Do not create another worktree or commit, reset, or clean the shared checkout; the parent owns integration."
    } else {
        mode(read_only, writable)
    }
}

pub(super) fn mode(read_only: bool, expects_changes: bool) -> &'static str {
    if read_only {
        "READ-ONLY TASK: inspect, analyze, and report. Do not modify files, run shell commands, or start autonomous implementation workflows."
    } else if !expects_changes {
        "VERIFICATION TASK: run the focused checks requested in the task inside the isolated worktree. Source edits are not required; report commands and outcomes as evidence."
    } else {
        "IMPORTANT: You MUST use tools to make changes. Do not just describe what to do - actually do it using the tools available."
    }
}

pub(super) fn tools(read_only: bool, expects_changes: bool) -> &'static str {
    if read_only {
        "Available tools:\n- read: Read file contents\n- glob: Find files by pattern\n- grep: Search file contents\n- tree/fileinfo/headtail: Inspect repository state\n- lsp/codesearch: Query code intelligence\n- webfetch/websearch: Research external context"
    } else if !expects_changes {
        "Available tools:\n- read/glob/grep/lsp: Inspect the isolated workspace\n- git: Read repository status and diffs; commit is unavailable\n- bash: Run only focused verification commands in the assigned workspace\n- File-editing and autonomous implementation workflows are unavailable"
    } else {
        "Available tools:\n- read/write/edit/multiedit/apply_patch: Inspect and modify files\n- glob/grep/lsp/codesearch: Search the assigned workspace\n- bash/git: Run scoped commands and repository operations\n- swarm_share: Coordinate results with peer swarm participants"
    }
}
