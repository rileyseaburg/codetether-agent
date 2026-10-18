// Public presentation API.
pub use agent_rail::agent_rail_spans;
pub use assembly::build_status_lines;
pub use badge_assembly::badge_spans;
pub use badge_source::BadgeSource;
pub use bus_badge::bus_status_badge_span;
pub use bus_label::bus_status_label_and_color;
pub use bus_view::BusStatus;
pub use clock_labels::{format_duration_ms, format_timestamp};
pub use codetether_chat_layout::STACK_WIDTH_THRESHOLD;
pub use elapsed_badge::elapsed_badge;
pub use metric_assembly::metric_spans;
pub use metric_source::MetricSource;
pub use session_label::session_model_label;
pub use timing::latency_badge_spans;
pub use timing_view::RequestTimings;
pub use turn_badge::turn_latency_badge;
