//! Live token, cache, context and cost presentation for terminal views.
//!
//! [TokenDisplay] reads shared telemetry and uses canonical pricing and theme
//! types to produce status spans and detailed usage text. [context_indicator]
//! formats current-turn context usage for callers with model snapshots.
mod cache;
mod construction;
mod context;
mod context_query;
mod context_style;
mod cost;
mod cost_span;
mod default;
mod detailed;
mod model_cache;
mod model_row;
mod models;
mod navigation;
mod providers;
mod status;
mod throughput;
pub use context::context_indicator;
/// Stateless live token and cost presenter.
/// # Examples
/// ```
/// let display = codetether_token_display::TokenDisplay::new();
/// assert_eq!(display.get_context_limit(""), None);
/// ```
pub struct TokenDisplay;
#[cfg(test)]
mod minimax_pricing_tests;

/// Context-window gauges with truecolor and named-color rendering.
pub mod context_gauge;
mod gauge_value;
/// Colored normalized sparkline samples.
pub mod sparkline_paint;
/// Throughput samples and their terminal sparkline.
pub mod throughput_sparkline;
mod token_budget_span;
mod token_rate_span;
/// Status span construction from token, throughput and context data.
pub mod token_spans;
mod token_usage_spans;
