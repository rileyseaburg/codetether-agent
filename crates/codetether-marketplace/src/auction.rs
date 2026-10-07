//! Auction mechanics for OKR-based work bidding.

use serde::{Deserialize, Serialize};

/// An auction for a KR (key result) work item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrAuction {
    pub auction_id: String,
    pub okr_id: String,
    pub kr_id: String,
    pub description: String,
    pub moonshot_score: f32,
    pub bids: Vec<super::bid::AgentBid>,
    pub status: AuctionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuctionStatus {
    Open,
    Evaluating,
    Awarded,
    Expired,
}

/// Resolve an auction — pick the best bid by expected value.
///
/// Bids with NaN expected values are excluded to prevent nondeterministic
/// winner selection via `partial_cmp` returning `None`.
pub fn resolve_auction(auction: &mut KrAuction) -> Option<String> {
    if auction.bids.is_empty() {
        return None;
    }
    let winner = auction
        .bids
        .iter()
        .filter(|b| !b.expected_value().is_nan())
        .max_by(|a, b| {
            a.expected_value()
                .partial_cmp(&b.expected_value())
                .unwrap_or(std::cmp::Ordering::Equal)
        })?;
    auction.status = AuctionStatus::Awarded;
    Some(winner.agent_id.clone())
}
