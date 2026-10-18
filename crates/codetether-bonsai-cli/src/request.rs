//! Translate CLI arguments into a direct provider request, without agent context.
use codetether_provider_core::{CompletionRequest, ContentPart, Message, Role};
pub(super) fn from_args(args: &super::BonsaiArgs) -> CompletionRequest {
    CompletionRequest {
        model: codetether_provider_bonsai::MODEL.into(),
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: args.prompt.clone(),
            }],
        }],
        tools: vec![],
        temperature: Some(args.temperature),
        top_p: None,
        max_tokens: Some(args.max_tokens),
        stop: vec![],
    }
}
