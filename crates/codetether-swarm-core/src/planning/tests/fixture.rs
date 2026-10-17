use super::*;
use std::{collections::VecDeque, sync::Mutex};
pub(super) const PARALLEL: &str = r#"{"subtasks":[{"name":"one","instruction":"inspect one"},{"name":"two","instruction":"inspect two"}]}"#;
pub(super) fn ok(text: &str) -> Result<String> {
    Ok(text.to_owned())
}
pub(super) struct Mock {
    replies: Mutex<VecDeque<Result<String>>>,
    pub(super) prompts: Mutex<Vec<String>>,
}
impl Mock {
    pub(super) fn new(replies: Vec<Result<String>>) -> Self {
        Self {
            replies: Mutex::new(replies.into()),
            prompts: Mutex::new(Vec::new()),
        }
    }
}
#[async_trait]
impl PlanCompletion for Mock {
    async fn complete(&self, prompt: String) -> Result<String> {
        self.prompts.lock().unwrap().push(prompt);
        self.replies
            .lock()
            .unwrap()
            .pop_front()
            .expect("extra request")
    }
}
