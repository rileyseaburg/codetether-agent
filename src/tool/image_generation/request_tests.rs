use super::request_body;

#[test]
fn generation_request_uses_upstream_defaults() {
    let (endpoint, body) = request_body::build("paint a lake", Vec::new());
    assert_eq!(endpoint, "images/generations");
    assert_eq!(body["model"], "gpt-image-2");
    assert_eq!(body["quality"], "auto");
    assert_eq!(body["size"], "auto");
    assert!(body.get("images").is_none());
}

#[test]
fn edit_request_wraps_image_urls() {
    let (endpoint, body) =
        request_body::build("change light", vec!["data:image/png;base64,eA==".into()]);
    assert_eq!(endpoint, "images/edits");
    assert_eq!(body["images"][0]["image_url"], "data:image/png;base64,eA==");
}

#[test]
fn default_registry_exposes_image_generation_names() {
    let registry = crate::tool::ToolRegistry::with_defaults();
    assert!(registry.contains("image_gen"));
    assert!(registry.contains("imagegen"));
}
