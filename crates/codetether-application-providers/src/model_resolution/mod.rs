//! RLM model selection using the application's deferred provider cache.
//!
//! [`resolve`] applies the configured model choice; [`resolve_with_score`]
//! also supplies host scoring signals. Both preserve the RLM core fallback
//! behavior and load the shared registry only when its policy requires it.
mod resolve;
mod scored;
pub use resolve::resolve;
pub use scored::resolve_with_score;
