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
#[cfg(test)]
mod packing_tests;
pub mod processing_badge;
pub mod status_hints;
pub mod status_text;
pub mod yolo_badge;

mod bus_badge;
mod bus_label;
mod bus_view;
mod clock_labels;
mod request_span;
mod session_label;
mod timing;
mod timing_active;
mod timing_idle;
mod timing_view;

mod agent_rail;
mod badge_assembly;
mod badge_providers;
mod badge_source;
mod elapsed_badge;
mod metric_assembly;
mod metric_providers;
mod metric_source;
mod turn_badge;

include!("exports.rs");

mod streaming_header;
mod title;
pub mod title_brand;

/// Session status and permissions report commands.
pub mod command;
