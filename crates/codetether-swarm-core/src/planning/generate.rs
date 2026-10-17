use super::*;
/// Generate and validate a plan, requesting at most one repair on invalid output.
///
/// # Arguments
/// * `task` - Original task, included in any repair prompt.
/// * `initial_prompt` - Decomposition prompt produced from the chosen strategy.
/// * `parallel_required` - Whether at least two concurrent root tasks are required.
/// * `completion` - Host adapter retaining provider/model request policy.
///
/// # Returns
/// A validated plan, or a single-task fallback when repair fails and parallelism is optional.
///
/// # Errors
/// Returns an initial provider error, or a failed repair when parallel roots are required.
///
/// # Examples
/// ```no_run
/// # async fn example(host: &dyn codetether_swarm_core::planning::PlanCompletion) -> anyhow::Result<()> {
/// let prompt = codetether_swarm_core::planning::decomposition_prompt::build(
///     "inspect", codetether_swarm_core::DecompositionStrategy::Automatic, 4, true,
/// );
/// let plan = codetether_swarm_core::planning::generate_plan("inspect", prompt, true, host).await?;
/// assert!(plan.len() >= 2);
/// # Ok(())
/// # }
/// ```
pub async fn generate_plan(
    task: &str,
    initial_prompt: String,
    parallel_required: bool,
    completion: &dyn PlanCompletion,
) -> Result<HashMap<String, SubTask>> {
    let text = completion.complete(initial_prompt).await?;

    tracing::debug!("Decomposition response: {}", text);

    let parsed =
        parse_decomposition(&text).and_then(|tasks| plan::validate(tasks, parallel_required));
    match parsed {
        Ok(plan) => Ok(plan),

        Err(error) => repair::repair(completion, task, &text, error, parallel_required).await,
    }
}
