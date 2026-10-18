//! Publish streaming and bus output before accumulating the final answer.
use super::{Context, collect::collect};
use codetether_provider_core::CompletionResponse;
/// Publish visible/thinking content and accumulate answer text.
/// # Arguments
/// * `context` — Output buffer, channels and current turn identity.
/// * `response` — Normalized provider completion.
/// # Returns
/// Sends available stream/bus events and appends text without tool-call preambles.
/// # Examples
/// ```no_run
/// # async fn publish(response: &codetether_provider_core::CompletionResponse) {
/// let mut output = String::new();
/// let context = codetether_prompt_loop::output::Context::<()> { output: &mut output, events: None, bus: None, agent: "build", turn_id: "turn", step: 1 };
/// codetether_prompt_loop::output::emit(context, response).await;
/// # }
/// ```
pub async fn emit<S>(context: Context<'_, S>, response: &CompletionResponse) {
    let thinking = collect(&response.message.content, true);
    let text = collect(&response.message.content, false);
    super::stream::emit(context.events, &thinking, &text).await;
    if !thinking.is_empty() {
        codetether_bus_core::assistant_events::thinking(
            context.bus,
            context.agent,
            context.turn_id,
            context.step,
            &thinking,
        );
    }
    if !text.is_empty() {
        codetether_bus_core::assistant_events::text(
            context.bus,
            context.agent,
            context.turn_id,
            &text,
        );
        // Text accompanying a tool call is a preamble, not the answer. Keeping
        // it produced answers made entirely of intent ("Now let me read...").
        if super::narration::is_preamble(&response.message.content) {
            tracing::debug!(
                step = context.step,
                "withholding pre-tool narration from final answer"
            );
        } else {
            context.output.push_str(&format!("{text}\n"));
        }
    }
}
