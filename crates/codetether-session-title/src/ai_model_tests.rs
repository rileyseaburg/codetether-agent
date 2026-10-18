use super::{clean_title, title_from_model};

#[path = "ai_model_test_provider.rs"]
mod test_provider;
use test_provider::SelectedProvider;

#[tokio::test]
async fn uses_selected_model() {
    let title = title_from_model(&SelectedProvider, "selected-model", "seed")
        .await
        .unwrap();
    assert_eq!(title.as_deref(), Some("Generated title"));
}

#[test]
fn cleans_model_output() {
    assert_eq!(clean_title("\"Fix build errors\"."), "Fix build errors");
    assert_eq!(
        clean_title("Add session naming\nextra"),
        "Add session naming"
    );
}
