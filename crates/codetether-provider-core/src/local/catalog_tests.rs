//! Catalog lookup tests: ids, context windows, and uniqueness.

use super::{NVIDIA_MODELS, context_window, find};

#[test]
fn nemotron_nano_8b_is_catalogued() {
    let model = find("nemotron-nano-8b-v1").expect("entry present");
    assert_eq!(model.arch, "llama");
    assert_eq!(model.context_window, 131_072);
    assert_eq!(context_window("nemotron-nano-8b-v1"), 131_072);
}

#[test]
fn unknown_ids_are_rejected() {
    assert!(find("nemotron-nano-12b-v2").is_none());
    assert!(find("").is_none());
    // Uncatalogued ids fall back to a conservative window.
    assert_eq!(context_window("nemotron-nano-12b-v2"), 8192);
}

#[test]
fn ids_are_unique() {
    for (index, model) in NVIDIA_MODELS.iter().enumerate() {
        assert!(
            !NVIDIA_MODELS[..index].iter().any(|m| m.id == model.id),
            "duplicate catalog id {}",
            model.id
        );
    }
}
