use super::{Role, fixtures::*, serializers};
#[test]
fn legacy_chat_user_images_preserve_order_and_image_only_pixels() {
    for serialize in serializers() {
        let mixed = message(Role::User, vec![text("before"), image(), text("after")]);
        let image_only = message(Role::User, vec![image()]);
        let output = serialize(&[mixed, image_only]);
        assert_eq!(output[0]["content"][0]["text"], "before");
        assert_eq!(output[0]["content"][1]["type"], "image_url");
        assert_eq!(output[0]["content"][1]["image_url"]["url"], PIXELS);
        assert_eq!(output[0]["content"][2]["text"], "after");
        assert_eq!(output[1]["content"][0]["image_url"]["url"], PIXELS);
        assert_eq!(output[1]["content"].as_array().unwrap().len(), 1);
    }
}
