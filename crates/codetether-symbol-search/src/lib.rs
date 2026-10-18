//! Workspace symbol picker state, parsing, ranking and terminal presentation.
//!
//! [SymbolEntry] identifies a source symbol; [SymbolSearchState] owns the query,
//! selection and cached results. Parse LSP tool text with [parse::output], rank
//! with [rank::apply], and draw the popup with [render_symbol_search].
//! The host supplies language-server requests and background scheduling.

mod cache;
mod entry;
pub mod parse;
pub mod rank;
mod render;
mod state;

pub use entry::SymbolEntry;
pub use render::render_symbol_search;
pub use state::{SymbolSearchMode, SymbolSearchState};
