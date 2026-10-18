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
