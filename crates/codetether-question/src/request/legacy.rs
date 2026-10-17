//! Convert legacy question/options/default arguments to one question.
use super::{Choice, Question};
use crate::Error;
use serde::Deserialize;
pub(super) fn decode(value: serde_json::Value) -> Result<Vec<Question>, Error> {
    #[derive(Deserialize)]
    struct Legacy {
        question: String,
        #[serde(default)]
        options: Vec<String>,
        default: Option<String>,
    }
    let old: Legacy = serde_json::from_value(value).map_err(|e| Error::Invalid(e.to_string()))?;
    let mut labels = old.options;
    if let Some(default) = old.default {
        labels.retain(|label| label != &default);
        labels.insert(0, default);
    }
    Ok(vec![Question {
        question: old.question,
        header: String::new(),
        multiple: false,
        options: labels
            .into_iter()
            .map(|label| Choice {
                label,
                description: String::new(),
            })
            .collect(),
    }])
}
