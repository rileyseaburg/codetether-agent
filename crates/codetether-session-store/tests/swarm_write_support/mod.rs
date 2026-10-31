//! The same (Message, PageKind) record format used by the session encoder.
mod verify;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_session_store::{Batch, Records, transaction};
pub use verify::counts;

pub fn write(path: &std::path::Path, id: &str) {
    for step in 0..100 {
        let message = Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: format!("{id} output {step}"),
            }],
        };
        let batch = Batch {
            id: id.into(),
            revision: step as i64,
            header: "{}".into(),
            messages: Records {
                start: step,
                end: step + 1,
                rows: vec![serde_json::to_string(&(message, PageKind::Conversation)).unwrap()],
            },
            tools: Records {
                start: 0,
                end: 0,
                rows: vec![],
            },
        };
        transaction::commit(path, &batch)
            .unwrap_or_else(|error| panic!("{id} step {step}: {error:#}"));
    }
}
