use crate::provider::{ContentPart, Message, Role};
use serde_json::{Value, json};
mod compatibility;
mod fixtures;
mod leading_packed;
mod packed_text;
mod packed_tools;
mod tools;
mod users;

fn serializers() -> [fn(&[Message]) -> Vec<Value>; 5] {
    use crate::provider::{
        copilot::CopilotProvider, google::GoogleProvider, moonshot::MoonshotProvider,
        openrouter::OpenRouterProvider, stepfun::StepFunProvider,
    };
    [
        GoogleProvider::convert_messages,
        MoonshotProvider::convert_messages,
        OpenRouterProvider::convert_messages,
        CopilotProvider::convert_messages,
        |messages| {
            StepFunProvider::new("unused".into())
                .unwrap()
                .convert_messages(messages)
        },
    ]
}

mod tool_image_first;
