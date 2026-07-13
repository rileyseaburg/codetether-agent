//! Runtime-mode and available-tool prompt sections.

pub(super) fn mode(read_only: bool) -> &'static str {
    if read_only {
        "READ-ONLY TASK: inspect, analyze, and report. Do not modify files, run shell commands, or start autonomous implementation workflows."
    } else {
        "IMPORTANT: You MUST use tools to make changes. Do not just describe what to do - actually do it using the tools available."
    }
}

pub(super) fn tools(read_only: bool) -> &'static str {
    if read_only {
        "Available tools:\n- read: Read file contents\n- glob: Find files by pattern\n- grep: Search file contents\n- tree/fileinfo/headtail/diff: Inspect repository state\n- lsp/codesearch: Query code intelligence\n- webfetch/websearch: Research external context"
    } else {
        "Available tools:\n- read/write/edit/multiedit: Inspect and modify files\n- glob/grep: Search the repository\n- bash: Run shell commands with the task cwd\n- prd/ralph/go: Run autonomous implementation workflows\n- swarm_share/agent: Coordinate with helper agents"
    }
}
