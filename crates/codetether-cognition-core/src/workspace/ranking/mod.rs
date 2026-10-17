//! Rank shared beliefs, unresolved attention and uncertainty summaries.
//!
//! [top_belief_ids], [rank_attention] and [top_uncertainties] preserve each
//! policy's existing ordering and limits. Callers own workspace updates.
mod attention;
mod belief;
mod uncertainty;
pub use attention::rank_attention;
pub use belief::top_belief_ids;
pub use uncertainty::top_uncertainties;
