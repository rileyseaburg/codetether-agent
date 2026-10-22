mod invocation;
mod publication;
use codetether_provider_core::Message;
#[derive(Default)]
pub struct State {
    pub messages: Vec<Message>,
    pub persisted: bool,
    pub spoof: bool,
}
pub fn call() -> codetether_prompt_loop::Call {
    codetether_prompt_loop::Call {
        id: "denied-call".into(),
        name: "exec_command".into(),
        input: serde_json::json!({"cmd":"probe old server"}),
    }
}
