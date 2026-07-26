use super::{args::ImagegenArgs, references};
use std::{collections::VecDeque, sync::Mutex};

#[tokio::test]
async fn conversation_images_take_priority_over_process_cache() {
    let args = ImagegenArgs {
        prompt: "edit it".into(),
        referenced_image_paths: None,
        num_last_images_to_include: Some(2),
        recent_images: vec!["user".into(), "generated".into()],
        session_id: None,
        call_id: None,
    };
    let cache = Mutex::new(VecDeque::from(["stale".into()]));
    let images = references::resolve(&args, &cache).await.unwrap();
    assert_eq!(images, ["user", "generated"]);
}

#[test]
fn runtime_images_are_not_model_facing_schema_fields() {
    assert!(
        super::schema::parameters()["properties"]
            .get("__ct_recent_images")
            .is_none()
    );
}