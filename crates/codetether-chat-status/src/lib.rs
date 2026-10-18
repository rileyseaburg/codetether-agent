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
pub use bus_badge::bus_status_badge_span;
pub use bus_label::bus_status_label_and_color;
pub use bus_view::BusStatus;
pub use clock_labels::{format_duration_ms, format_timestamp};
pub use session_label::session_model_label;
pub use timing::latency_badge_spans;
pub use timing_view::RequestTimings;
