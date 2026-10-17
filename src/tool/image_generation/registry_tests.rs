//! Application registry aliases for the image-generation tool.

#[test]
fn default_registry_exposes_image_generation_names() {
    let registry = crate::tool::ToolRegistry::with_defaults();
    assert!(registry.contains("image_gen"));
    assert!(registry.contains("imagegen"));
}
