//! Asynchronous wrapper around the blocking terminal prompt editor.

mod completion;
mod editor;
mod render;
mod state;
mod terminal;

pub(super) async fn read(workspace: std::path::PathBuf) -> anyhow::Result<Option<String>> {
    tokio::task::spawn_blocking(move || editor::read(&workspace)).await?
}
