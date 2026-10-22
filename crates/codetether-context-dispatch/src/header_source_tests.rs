use super::*;
use codetether_provider_core::Role;

struct Fixture(Vec<Message>);
#[async_trait]
impl Source for Fixture {
    fn messages(&self) -> &[Message] {
        &self.0
    }
    fn pages(&self) -> &[PageKind] {
        &[]
    }
    async fn prepared_index(&self) -> SummaryIndex {
        SummaryIndex::new()
    }
}

#[test]
fn missing_attachments_and_nontext_results_never_claim_complete_header_coverage() {
    let text = ContentPart::Text {
        text: "full text".into(),
    };
    let source = Fixture(vec![
        Message {
            role: Role::User,
            content: vec![text.clone()],
        },
        Message {
            role: Role::User,
            content: vec![
                text,
                ContentPart::Image {
                    url: "image".into(),
                    mime_type: None,
                },
            ],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "c".into(),
                content: "result".into(),
            }],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: String::new(),
            }],
        },
    ]);
    let covered = HeaderSource(&source);
    assert!(covered.pinned_source_in_header(0));
    for index in 1..5 {
        assert!(!covered.pinned_source_in_header(index));
    }
}
