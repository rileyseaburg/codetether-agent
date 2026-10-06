//! Model IDs shared by Vertex GLM requests and harness routing identity.
pub(crate) fn normalize(model: &str) -> String {
    if model.starts_with("zai-org/") {
        model.to_owned()
    } else {
        format!("zai-org/{model}-maas")
    }
}
