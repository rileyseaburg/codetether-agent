//! # Token telemetry
//!
//! Everything related to LLM token accounting.
//!
//! | File | Responsibility |
//! |---|---|
//! | [`totals`]   | [`TokenTotals`] — simple input/output pair |
//! | [`counts`]   | [`TokenCounts`] — wire-format input/output pair |
//! | [`snapshot`] | [`TokenUsageSnapshot`] + [`GlobalTokenSnapshot`] |
//! | [`counter`]  | [`AtomicTokenCounter`] — the lock-light global counter |

pub mod counter;
pub mod counts;
pub mod snapshot;
pub mod totals;

pub use counter::AtomicTokenCounter;
pub use counts::TokenCounts;
pub use snapshot::{GlobalTokenSnapshot, TokenUsageSnapshot};
pub use totals::TokenTotals;
