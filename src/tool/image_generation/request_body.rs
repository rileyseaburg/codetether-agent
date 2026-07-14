use serde_json::{Value, json};

pub(super) fn build(prompt: &str, images: Vec<String>) -> (&'static str, Value) {
    let common = json!({"model": super::IMAGE_MODEL, "prompt": prompt,
        "background": "auto", "quality": "auto", "size": "auto"});
    if images.is_empty() {
        return ("images/generations", common);
    }
    let mut body = common;
    body["images"] = json!(
        images
            .into_iter()
            .map(|image_url| json!({"image_url": image_url}))
            .collect::<Vec<_>>()
    );
    ("images/edits", body)
}
