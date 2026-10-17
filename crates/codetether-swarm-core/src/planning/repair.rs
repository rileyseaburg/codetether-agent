use super::*;
pub(super) async fn repair(
    completion: &dyn PlanCompletion,
    task: &str,
    text: &str,
    error: anyhow::Error,
    parallel_required: bool,
) -> Result<HashMap<String, SubTask>> {
    tracing::warn!(%error, response = %text, "Invalid swarm plan; requesting one repair");
    let prompt = repair_prompt::build(task, &text, &error.to_string(), parallel_required);
    match completion
        .complete(prompt)
        .await
        .and_then(|text| parse_decomposition(&text))
        .and_then(|tasks| plan::validate(tasks, parallel_required))
    {
        Ok(plan) => Ok(plan),
        Err(repair_error) => {
            if parallel_required {
                return Err(repair_error
                    .context("Swarm decomposition did not produce concurrent root tasks"));
            }
            tracing::warn!(%repair_error, "Swarm plan repair failed; using one truthful task");
            Ok(plan::single(task))
        }
    }
}
