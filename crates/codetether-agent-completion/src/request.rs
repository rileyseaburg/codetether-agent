//! Assemble model, policy-filtered tools and native prompt for one completion.
use crate::*;
pub(crate) async fn complete_with_context<H: Host>(
    host: &H,
    session: &H::Session,
) -> anyhow::Result<CompletionResponse> {
    let model = crate::model::default_for(host);
    let tools: Vec<_> = host
        .definitions()
        .into_iter()
        .filter(|tool| H::tool_available(session, &tool.name))
        .collect();
    let system_prompt = host.system_prompt(session);
    H::complete(Request {
        provider: Arc::clone(host.provider()),
        session,
        model: &model,
        system_prompt: &system_prompt,
        tools: &tools,
        temperature: host.info().temperature,
        top_p: host.info().top_p,
    })
    .await
}
