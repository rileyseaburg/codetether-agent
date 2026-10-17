//! Pricing lookup and benchmark result submission over HTTP.
//!
//! Call fetch_pricing for optional model prices, or submit_results for
//! best-effort publication. Both operations retain bounded request timeouts.
mod fetch_pricing;
mod send_submission;
mod submit_results;
pub use fetch_pricing::fetch_pricing;
pub use submit_results::submit_results;
