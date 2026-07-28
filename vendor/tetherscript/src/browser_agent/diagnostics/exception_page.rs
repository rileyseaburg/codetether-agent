//! Page-error exception classification.

use crate::browser_agent::events::PageErrorEvent;

use super::exception_types::RuntimeException;

pub fn collect(errors: &[PageErrorEvent]) -> Vec<RuntimeException> {
    errors
        .iter()
        .map(|error| RuntimeException {
            action: error.action.clone(),
            message: error.message.clone(),
            kind: super::exception_classify::kind(&error.message),
        })
        .collect()
}
