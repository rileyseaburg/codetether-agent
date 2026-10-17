include!("user_image_serialization_text.rs");
#[test]
fn user_image_serialization_does_not_change_instruction_roles() {
    for (role, name) in [(Role::System, "system"), (Role::Developer, "developer")] {
        let message = Message {
            role,
            content: vec![
                ContentPart::Text {
                    text: "instruction".into(),
                },
                user_image_part(USER_IMAGE_DATA_URL),
            ],
        };
        let expected = json!({ "role": name, "content": "instruction" });
        assert_eq!(
            serialized_user_image_message(message.clone(), false),
            expected
        );
        if name == "system" {
            let mut input = Vec::new();
            OpenAiCodexProvider::append_responses_message(
                &message,
                &mut input,
                &mut std::collections::HashSet::new(),
            );
            assert!(input.is_empty());
        } else {
            assert_eq!(serialized_user_image_message(message, true), expected);
        }
    }
}
