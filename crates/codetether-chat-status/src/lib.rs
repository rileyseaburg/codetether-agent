//! Chat-status hints, badges, and responsive line assembly.
//!
//! Build badges from display values and supply metric spans to
//! [build_status_lines]. Native code owns application state and metric sampling.
mod assembly;
pub mod auto_apply;
pub mod compact_hints;
mod hint_header;
mod hint_keys;
mod hint_session;
pub mod images_badge;
pub mod kb_style;
pub mod processing_badge;
pub mod status_hints;
pub mod status_text;
pub mod yolo_badge;
pub use assembly::build_status_lines;
pub use codetether_chat_layout::STACK_WIDTH_THRESHOLD;
#[cfg(test)]
mod packing_tests;
