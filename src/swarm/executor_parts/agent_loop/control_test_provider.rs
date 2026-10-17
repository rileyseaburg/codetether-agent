//! Offline provider that steers or cancels during its first final response.

use crate::provider::*;
use crate::session::runtime::ControlHandle;
use async_trait::async_trait;
use futures::stream::BoxStream;
use std::sync::Mutex;

pub(super) struct Mock {
    pub control: ControlHandle,
    pub cancel: bool,
    pub requests: Mutex<Vec<CompletionRequest>>,
}

#[async_trait]
impl Provider for Mock {
    fn name(&self) -> &str {
        "mock"
    }
    async fn list_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        Ok(vec![])
    }
    async fn complete(&self, request: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        let mut requests = self.requests.lock().unwrap();
        let first = requests.is_empty();
        requests.push(request);
        if first {
            if self.cancel {
                self.control.cancel();
            } else {
                self.control.publish_input(Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: "Inspect cloned repos".into(),
                    }],
                });
            }
        }
        Ok(super::control_test_response::response(self.cancel))
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> anyhow::Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("tests use complete")
    }
}
