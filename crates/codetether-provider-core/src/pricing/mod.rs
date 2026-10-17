//! Model price and prompt-cache matching without telemetry dependencies.
//!
//! Prices are the existing best-effort values. Matching order and fallback
//! behavior are retained; callers supply the model identifier.
mod cache_read_multiplier;
mod cache_write_multiplier;
mod pricing_for_model;
pub use cache_read_multiplier::cache_read_multiplier;
pub use cache_write_multiplier::cache_write_multiplier;
pub use pricing_for_model::pricing_for_model;
