//! Existing page policy regression.
use super::*;

#[test]
fn classify_routes_system_to_bootstrap() {
    let msg = Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: "you are…".to_string(),
        }],
    };
    assert_eq!(classify(&msg), PageKind::Bootstrap);
}
