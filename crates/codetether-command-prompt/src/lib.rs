//! Asynchronous wrapper around the blocking terminal prompt editor.
//!
//! [read] edits one mux control command with character-aware cursor movement,
//! safe single-line paste, and directory completion for `cd` and `new`.
//! A terminal guard restores bracketed-paste and raw-mode state on return.

mod completion;
mod editor;
mod input_event;
mod paste;
mod render;
mod state;
mod terminal;

/// Read one edited command using the current terminal on a blocking worker.
/// # Arguments
/// `workspace` anchors relative directory completions.
/// # Returns
/// The submitted line, or None when the user cancels or requests end-of-input.
/// # Errors
/// Reports worker, terminal mode, event-reading and prompt-rendering failures.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _line = codetether_command_prompt::read(std::path::PathBuf::from(".")).await?;
/// # Ok(()) }
/// ```
pub async fn read(workspace: std::path::PathBuf) -> anyhow::Result<Option<String>> {
    tokio::task::spawn_blocking(move || editor::read(&workspace)).await?
}

mod editor_helpers;
mod state_edit;
mod state_move;
