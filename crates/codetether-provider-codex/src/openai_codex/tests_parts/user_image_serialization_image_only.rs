#[test]
fn user_image_serialization_retains_image_only_pixels() {
    for responses in [true, false] {
        for empty_text in [true, false] {
            let mut content = vec![user_image_part(USER_IMAGE_DATA_URL)];
            if empty_text {
                content.insert(
                    0,
                    ContentPart::Text {
                        text: String::new(),
                    },
                );
            }
            let wire = serialized_user_image_message(
                Message {
                    role: Role::User,
                    content,
                },
                responses,
            );
            let nodes = wire["content"].as_array().unwrap();
            assert_eq!(nodes.len(), if empty_text { 2 } else { 1 });
            let image = nodes.last().unwrap();
            let expected = if responses {
                json!({ "type": "input_image", "image_url": USER_IMAGE_DATA_URL })
            } else {
                json!({ "type": "image_url", "image_url": { "url": USER_IMAGE_DATA_URL } })
            };
            assert_eq!(image, &expected);
            assert!(image.get("text").is_none());
            if empty_text {
                assert_eq!(nodes[0]["text"], "");
            }
        }
    }
}
