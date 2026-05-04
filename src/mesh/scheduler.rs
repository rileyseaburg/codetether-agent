//! Mesh task scheduler — picks the best peer for each subtask.

use super::cost_card::CostCard;

/// Select the best peer for a task given available cost cards.
pub fn select_peer(cards: &[CostCard], requires_gpu: bool) -> Option<&CostCard> {
    let eligible: Vec<&CostCard> = cards.iter()
        .filter(|c| {
            c.current_load < 0.95
            && c.free_cores > 0
            && (!requires_gpu || c.gpu_available)
        })
        .collect();
    eligible.into_iter().min_by(|a, b| {
        a.scheduling_score().partial_cmp(&b.scheduling_score()).unwrap_or(std::cmp::Ordering::Equal)
    })
}

/// Plan task distribution across mesh peers.
pub fn plan_distribution(tasks: &[String], cards: &[CostCard]) -> Vec<(String, String)> {
    let mut assignments = Vec::new();
    for task in tasks {
        if let Some(peer) = select_peer(cards, false) {
            assignments.push((task.clone(), peer.peer_id.clone()));
        }
    }
    assignments
}
