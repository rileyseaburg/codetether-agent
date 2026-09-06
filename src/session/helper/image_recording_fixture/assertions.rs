//! Assert call-associated images survive the actual recording text digest.

use crate::provider::{ContentPart, Role};
use crate::session::Session;

pub(in crate::session::helper) fn assert_recorded(session: &Session) {
    assert_eq!(session.messages.len(), 2, "no standalone image messages");
    for (message, (id, status)) in session
        .messages
        .iter()
        .zip([("image-success", "success"), ("image-failure", "failure")])
    {
        assert_eq!(message.role, Role::Tool);
        let [
            ContentPart::ToolResult {
                tool_call_id,
                content,
            },
            first,
            second,
        ] = message.content.as_slice()
        else {
            panic!("expected one tool result followed by exactly two images");
        };
        assert_eq!(tool_call_id, id);
        assert!(content.contains(&format!("- status: {status}")));
        assert!(content.contains("image evidence"));
        assert!(content.contains("runtime digest"));
        assert!(content.len() < 5000, "long text must be compacted");
        assert!(!content.contains("base64"));
        for (part, (expected_url, expected_mime)) in
            [first, second].into_iter().zip(super::tool::IMAGES)
        {
            assert!(!content.contains(expected_url.split(',').nth(1).unwrap()));
            assert!(matches!(part, ContentPart::Image { url, mime_type }
                if url == expected_url && mime_type.as_deref() == Some(expected_mime)));
        }
    }
}
