use codetether_prompt_loop::response::progress::Host;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_repeat_guard::ProgressLedger;
#[derive(Default)]
pub struct State {
    pub ledger: ProgressLedger,
    pub messages: Vec<Message>,
}
impl Host for State {
    fn evidence(&mut self) -> &mut ProgressLedger {
        &mut self.ledger
    }
    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
pub fn result(state: &mut State, index: usize, input: serde_json::Value) {
    let id = index.to_string();
    state.ledger.observe_message(
        &Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: id.clone(),
                name: "bash".into(),
                arguments: input.to_string(),
                thought_signature: None,
            }],
        },
        index * 2,
    );
    state.ledger.observe_message(
        &codetether_tool_core::messages::tool_result_with_status(
            id,
            "bash",
            true,
            "same output".into(),
        ),
        index * 2 + 1,
    );
}
