// Shell-command tools and their shared persistent-session runtime.

pub mod bash;
pub mod collaboration;
pub(crate) mod command_pty;
pub(crate) mod command_session;
pub mod exec_command;
use codetether_process_tree as process_tree;
pub(crate) mod shell_command_guard;
pub mod temp_write_guard;
/// Estimates file token cost before the agent reads it.
pub mod token_count;
pub mod write_stdin;
// Approval-time reconstruction of files those tools would mutate.
pub(crate) mod proposed_content;
