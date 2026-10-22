//! Tool identifiers retained by compact runtime profiles.

#[path = "catalog_ids/discovery.rs"]
mod discovery;
pub(super) use discovery::DISCOVERY;

pub(super) const CODING: &[&str] = &[
    "rust_build_plan",
    "agent",
    "apply_patch",
    "browserctl",
    "close_agent",
    "codesearch",
    "computer_use",
    "create_goal",
    "edit_goal",
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
