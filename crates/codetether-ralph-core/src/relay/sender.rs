//! Select the previous participant for an ordered relay turn.
/// Return the user for the initial turn, or the prior ordered agent.
///
/// # Arguments
/// `ordered_agents` supplies turn order; `idx` and `round` identify the current turn.
/// # Returns
/// An owned sender name.
/// # Panics
/// Panics if the required predecessor index is outside `ordered_agents`.
/// # Examples
/// ```
/// assert_eq!(codetether_ralph_core::relay::sender(&["coder".into()], 0, 1), "user");
/// ```

pub fn sender(ordered_agents: &[String], idx: usize, round: usize) -> String {
    let from = if idx == 0 {
        if round == 1 {
            "user".to_string()
        } else {
            ordered_agents[ordered_agents.len() - 1].clone()
        }
    } else {
        ordered_agents[idx - 1].clone()
    };

    from
}
