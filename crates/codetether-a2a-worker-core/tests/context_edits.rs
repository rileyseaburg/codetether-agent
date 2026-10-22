use async_trait::async_trait;
use codetether_a2a_worker_core::{
    execution_policy::AutoApprove,
    session_protocol::tool_call::{ToolCallHost, execute_tool_call},
};
use codetether_tool_core::ToolResult;

#[derive(Default)]
struct Host {
    events: Vec<&'static str>,
    results: Vec<ToolResult>,
    fail_save: bool,
    fail_refresh: bool,
    fail_tool: bool,
    repeated: bool,
}
#[async_trait]
impl codetether_a2a_worker_core::session_protocol::response::ResponseHost for Host {
    async fn ensure_running(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn record_usage(&mut self, _: &str, _: u64, _: u64) {}
    fn append_message(&mut self, _: codetether_provider_core::Message) {
        self.events.push("assistant");
    }
    async fn execute_tool_call(
        &mut self,
        call: codetether_a2a_worker_core::session_protocol::ToolCall,
    ) -> anyhow::Result<()> {
        execute_tool_call(self, AutoApprove::All, &None, call).await
    }
}
#[async_trait]
impl ToolCallHost for Host {
    fn repeated_edit(&self, _: &str, _: &serde_json::Value) -> Option<String> {
        self.repeated
            .then(|| "REPEATED_EDIT_BLOCKED: unchanged failure".into())
    }
    fn prepare_input(&mut self, input: &serde_json::Value) -> serde_json::Value {
        input.clone()
    }
    async fn persist_context(&mut self) -> anyhow::Result<()> {
        self.events.push("save");
        if self.fail_save {
            anyhow::bail!("save failed");
        }
        Ok(())
    }
    async fn refresh_context(&mut self) -> anyhow::Result<()> {
        self.events.push("refresh");
        if self.fail_refresh {
            anyhow::bail!("refresh failed");
        }
        Ok(())
    }
    async fn run_tool(&mut self, _: &str, _: serde_json::Value) -> ToolResult {
        self.events.push("execute");
        if self.fail_tool {
            ToolResult::error("tool failure")
        } else {
            ToolResult::success("original result")
        }
    }
    fn record_result(&mut self, _: String, result: ToolResult) {
        self.events.push("record");
        self.results.push(result);
    }
}
#[tokio::test]
async fn repeated_edit_evidence_prevents_execution_without_affecting_recording() {
    let mut host = Host {
        repeated: true,
        ..Default::default()
    };
    run(&mut host, "edit", AutoApprove::All).await.unwrap();
    assert_eq!(host.events, ["record"]);
    assert!(!host.results[0].success);
    assert!(host.results[0].output.contains("REPEATED_EDIT_BLOCKED"));
}
async fn run(host: &mut Host, name: &str, policy: AutoApprove) -> anyhow::Result<()> {
    execute_tool_call(
        host,
        policy,
        &None,
        ("c".into(), name.into(), serde_json::json!({})),
    )
    .await
}

#[tokio::test]
async fn context_edits_save_then_refresh_before_recording_results() {
    let mut host = Host::default();
    run(&mut host, "context_pin", AutoApprove::All)
        .await
        .unwrap();
    assert_eq!(host.events, ["save", "execute", "refresh", "record"]);
    assert!(host.results[0].success);
    let mut failed = Host {
        fail_tool: true,
        ..Default::default()
    };
    run(&mut failed, "context_summarize", AutoApprove::All)
        .await
        .unwrap();
    assert_eq!(failed.events, ["save", "execute", "record"]);
}

#[tokio::test]
async fn failure_before_execution_or_during_refresh_is_reported_without_losing_output() {
    let mut before = Host {
        fail_save: true,
        ..Default::default()
    };
    assert!(
        run(&mut before, "context_pin", AutoApprove::All)
            .await
            .is_err()
    );
    assert_eq!(before.events, ["save", "record"]);
    assert!(!before.results[0].success);
    let mut after = Host {
        fail_refresh: true,
        ..Default::default()
    };
    assert!(
        run(&mut after, "context_pin", AutoApprove::All)
            .await
            .is_err()
    );
    assert!(!after.results[0].success);
    assert!(after.results[0].output.contains("original result"));
    assert_eq!(
        after.results[0].metadata.get("context_edit_committed"),
        Some(&serde_json::json!(true))
    );
}

#[tokio::test]
async fn ordinary_reads_and_denied_edits_skip_persistence_hooks() {
    let mut read = Host::default();
    run(&mut read, "read", AutoApprove::Safe).await.unwrap();
    assert_eq!(read.events, ["execute", "record"]);
    let mut denied = Host::default();
    run(&mut denied, "context_pin", AutoApprove::None)
        .await
        .unwrap();
    assert_eq!(denied.events, ["record"]);
    assert!(!denied.results[0].success);
}

#[tokio::test]
async fn failed_refresh_stops_later_tools_in_the_same_model_response() {
    use codetether_provider_core::{CompletionResponse, ContentPart, FinishReason, Message, Role};
    let mut host = Host {
        fail_refresh: true,
        ..Default::default()
    };
    let content = ["context_pin", "read"]
        .iter()
        .enumerate()
        .map(|(index, name)| ContentPart::ToolCall {
            id: index.to_string(),
            name: (*name).into(),
            arguments: "{}".into(),
            thought_signature: None,
        })
        .collect();
    let response = CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: Default::default(),
        finish_reason: FinishReason::Stop,
    };
    let outcome = codetether_a2a_worker_core::session_protocol::response::process_response(
        &mut host,
        "fixture",
        &None,
        &mut String::new(),
        response,
    )
    .await;
    assert!(outcome.is_err());
    assert_eq!(
        host.events,
        ["assistant", "save", "execute", "refresh", "record"]
    );
    assert_eq!(host.results.len(), 1);
}
