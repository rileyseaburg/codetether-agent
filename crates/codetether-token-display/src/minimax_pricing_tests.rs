//! Preserve the MiniMax rates formerly tested through the TUI helper.

#[test]
fn minimax_m25_pricing_estimate_matches_rates() {
    let cost = crate::TokenDisplay::new()
        .calculate_cost_for_tokens("minimax/MiniMax-M2.5", 1_000_000, 1_000_000)
        .total_cost;
    assert!((cost - 1.5).abs() < 1e-9); // $0.3 + $1.2 = $1.5
}

#[test]
fn minimax_m25_highspeed_pricing() {
    let cost = crate::TokenDisplay::new()
        .calculate_cost_for_tokens("MiniMax-M2.5-highspeed", 1_000_000, 1_000_000)
        .total_cost;
    assert!((cost - 3.0).abs() < 1e-9); // $0.6 + $2.4 = $3.0
}
