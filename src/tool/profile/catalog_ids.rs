//! Tool identifiers retained by compact runtime profiles.

pub(super) const DISCOVERY: &[&str] = &[
    "agent",
    "apply_patch",
    "codesearch",
    "exec_command",
    "glob",
    "grep",
    "list",
    "lsp",
    "read",
    "rg",
    "session_task",
    "write_stdin",
];

pub(super) const CODING: &[&str] = &[
    "agent",
    "apply_patch",
    "browserctl",
    "close_agent",
    "codesearch",
    "computer_use",
    "create_goal",
    "exec_command",
    "followup_task",
    "glob",
    "grep",
    "get_goal",
    "image",
    "image_gen",
    "interrupt_agent",
    "list",
    "list_agents",
    "lsp",
    "read",
    "resume_agent",
    "rg",
    "send_input",
    "send_message",
    "session_task",
    "skill",
    "spawn_agent",
    "update_goal",
    "wait_agent",
    "webfetch",
    "websearch",
    "write_stdin",
];

// Keep the legacy profile fail-closed: agents no longer control mux sessions.
pub(super) const MUX_MANAGER: &[&str] = &[];
