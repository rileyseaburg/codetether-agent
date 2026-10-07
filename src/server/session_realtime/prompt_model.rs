//! Apply an explicit client model before resolving a realtime turn's provider.

use crate::session::SessionMetadata;

/// Keep older clients compatible; explicit qualified model selectors replace the default.
pub(super) fn apply(metadata: &mut SessionMetadata, model: Option<String>) {
    if let Some(model) = model.filter(|model| !model.trim().is_empty()) {
        metadata.model = Some(model);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_model_replaces_previous_choice() {
        let mut metadata = SessionMetadata {
            model: Some("provider/old".into()),
            ..Default::default()
        };
        apply(&mut metadata, Some("provider/chosen".into()));
        assert_eq!(metadata.model.as_deref(), Some("provider/chosen"));
    }

    #[test]
    fn omitted_or_blank_model_preserves_session_choice() {
        let mut metadata = SessionMetadata {
            model: Some("provider/current".into()),
            ..Default::default()
        };
        for model in [None, Some("  ".into())] {
            apply(&mut metadata, model);
        }
        assert_eq!(metadata.model.as_deref(), Some("provider/current"));
    }
}
