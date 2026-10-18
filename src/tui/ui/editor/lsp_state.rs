//! Shared editor LSP state specialized to the application manager.
pub type EditorLspState = codetether_editor_lsp::EditorLspState<crate::lsp::LspManager>;
