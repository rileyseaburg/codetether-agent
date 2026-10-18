//! Chat panel geometry, status packing, visible slices and scrollbars.
//!
//! [compute_chat_chunks] accepts display inputs without application state.
//! [pack_spans] and [visible_lines] arrange already-rendered terminal content.
mod layout_chunks;
mod layout_compute;
pub mod scroll_indicator;
pub mod scrollbar_render;
pub mod status_glue;
mod status_height;
pub mod status_pack;
mod viewport;
pub use layout_chunks::ChatChunks;
pub use layout_compute::compute_chat_chunks;
pub use status_pack::{pack_spans, pack_stacked};
pub use viewport::visible_lines;
/// Width below which chat status uses three rows instead of one.
pub const STACK_WIDTH_THRESHOLD: u16 = 180;

mod hit;
pub use hit::ChatHit;

mod scroll_down;
mod scroll_position;
mod scroll_state;
mod scroll_up;
pub use scroll_state::ChatScrollState;
/// Sentinel position used to follow the latest chat output.
pub const SCROLL_BOTTOM: usize = 1_000_000;

/// Responsive webview layout and layout-mode selection.
pub mod webview;
