//! Sequential Ralph prompt handling.
use crate::*;
/// Build a story prompt enriched with accumulated bus learnings.
///
/// # Arguments
/// Receives the live native host and the current story or response when applicable.
/// # Returns
/// The enriched story prompt.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &impl codetether_ralph_sequential::Host, story: &codetether_ralph_core::UserStory) -> anyhow::Result<()> {
/// let prompt = codetether_ralph_sequential::sequential_prompt(host, story);
/// # Ok(())
/// # }
/// ```
pub fn sequential_prompt(host: &impl Host, story: &UserStory) -> String {
    // Collect accumulated bus learnings for prompt injection
    let bus_learnings = host.bus_collect_learnings();

    // Build the prompt (with bus learnings if available)
    let prompt = if bus_learnings.is_empty() {
        host.build_prompt(story)
    } else {
        let mut p = host.build_prompt(story);
        p.push_str("\n## Learnings from Previous Iterations:\n");
        for l in &bus_learnings {
            p.push_str(&format!("- {l}\n"));
        }
        p
    };

    prompt
}
