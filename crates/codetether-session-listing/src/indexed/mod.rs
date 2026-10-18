//! Metadata-only database queries using the application's full header decoder.
//!
//! [`IndexedSource`] combines the standard session directory with the indexed
//! store. [`Header`] preserves the application's serde validation and summary
//! projection without requiring this crate to own its runtime session type.
mod header;
mod source;
pub use header::Header;
use std::marker::PhantomData;

/// Standard indexed listing source using `H` to decode each stored header.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_session_listing::indexed::Header>() -> anyhow::Result<()> {
/// let _sessions = codetether_session_listing::list_sessions::<
///     codetether_session_listing::indexed::IndexedSource<H>,
/// >().await?;
/// # Ok(()) }
/// ```
pub struct IndexedSource<H>(PhantomData<fn() -> H>);
