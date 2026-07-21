use super::message;
use crate::provider::Role;

#[test]
fn corrective_guidance_is_not_attributed_to_the_user() {
    let message = message("keep working");

    assert_eq!(message.role, Role::Developer);
    assert_eq!(message.content.len(), 1);
    let crate::provider::ContentPart::Text { text } = &message.content[0] else {
        panic!("corrective guidance must be text");
    };
    assert_eq!(text, "keep working");
}
