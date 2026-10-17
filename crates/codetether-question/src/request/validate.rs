//! Validate batch shape, prompt/header bounds, and distinct choice labels.
use super::Question;
use crate::Error;
pub(super) fn validate(questions: &[Question]) -> Result<(), Error> {
    if questions.is_empty() {
        return Err(Error::Invalid("questions must not be empty".into()));
    }
    for q in questions {
        let mut labels = std::collections::HashSet::new();
        if q.question.trim().is_empty() || q.header.chars().count() > 30 {
            return Err(Error::Invalid(
                "question must be nonempty and header at most 30 characters".into(),
            ));
        }
        for option in &q.options {
            if option.label.trim().is_empty() || !labels.insert(option.label.trim()) {
                return Err(Error::Invalid(
                    "choice labels must be nonempty and unique".into(),
                ));
            }
        }
    }
    Ok(())
}
