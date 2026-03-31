use super::{Agent, AgentResponse, ToolUse};
use crate::provider::{CompletionRequest, ContentPart, Message, Role};
use crate::session::Session;
use anyhow::{Result, bail};

type PendingToolCall = (String, String, String);

impl Agent {
    /// Execute a prompt and return the response
    pub async fn execute(&self, session: &mut Session, prompt: &str) -> Result<AgentResponse> {
        session.add_message(Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: prompt.to_string(),
            }],
        });

        let max_steps = self.info.max_steps.unwrap_or(100);

        for _step in 1..=max_steps {
            let response = self
                .provider
                .complete(self.build_completion_request(session))
                .await?;
            session.add_message(response.message.clone());

            let tool_calls = collect_tool_calls(&response.message);
            if tool_calls.is_empty() {
                return Ok(AgentResponse {
                    text: response_text(&response.message),
                    tool_uses: session.tool_uses.clone(),
                    usage: session.usage.clone(),
                });
            }

            self.execute_tool_calls(session, tool_calls).await;
        }

        bail!("Exceeded maximum steps ({max_steps})");
    }

    fn build_completion_request(&self, session: &Session) -> CompletionRequest {
        CompletionRequest {
            messages: self.build_messages(session),
            tools: self.tools.definitions(),
            model: self.default_model(),
            temperature: self.info.temperature,
            top_p: self.info.top_p,
            max_tokens: None,
            stop: vec![],
        }
    }

    /// Build the full message list including system prompt
    fn build_messages(&self, session: &Session) -> Vec<Message> {
        let mut messages = vec![Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: self.system_prompt.clone(),
            }],
        }];
        messages.extend(session.messages.clone());
        messages
    }

    fn default_model(&self) -> String {
        self.info
            .model
            .clone()
            .unwrap_or_else(|| match self.provider.name() {
                "zhipuai" | "zai" => "glm-5".to_string(),
                "openrouter" => "z-ai/glm-5".to_string(),
                _ => "glm-5".to_string(),
            })
    }

    async fn execute_tool_calls(&self, session: &mut Session, tool_calls: Vec<PendingToolCall>) {
        for (id, name, arguments) in tool_calls {
            let result = self.execute_tool(&name, &arguments).await;

            session.tool_uses.push(ToolUse {
                id: id.clone(),
                name: name.clone(),
                input: arguments.clone(),
                output: result.output.clone(),
                success: result.success,
            });

            session.add_message(Message {
                role: Role::Tool,
                content: vec![ContentPart::ToolResult {
                    tool_call_id: id,
                    content: result.output,
                }],
            });
        }
    }
}

fn collect_tool_calls(message: &Message) -> Vec<PendingToolCall> {
    message
        .content
        .iter()
        .filter_map(extract_tool_call)
        .collect()
}

fn extract_tool_call(part: &ContentPart) -> Option<PendingToolCall> {
    match part {
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            ..
        } => Some((id.clone(), name.clone(), arguments.clone())),
        _ => None,
    }
}

fn response_text(message: &Message) -> String {
    message
        .content
        .iter()
        .filter_map(extract_text)
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_text(part: &ContentPart) -> Option<String> {
    match part {
        ContentPart::Text { text } => Some(text.clone()),
        _ => None,
    }
}
