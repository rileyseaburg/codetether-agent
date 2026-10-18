//! Message-panel scrolling, drawing and visible-line hit tracking.
//!
//! Implement [Scroll] for history anchoring and [Host] for title, accent and hit
//! state. [render_owned] draws an owned buffer; [render_ref] draws its visible slice.
mod host;
mod render;
mod scroll;
pub use host::{Host, Scroll};
pub use render::{render_owned, render_ref};
pub use scroll::{clamp, reset_height_memo};
