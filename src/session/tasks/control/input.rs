//! Payload bounds for explicit user goal changes.

use super::error::GoalControlError as Error;
use crate::session::tasks::{GoalEdit, GoalEditAction};

/// Reject malformed fields before a native edit reaches the journal.
pub(super) fn validate(edit: &GoalEdit) -> Result<(), Error> {
    if let Some(text) = &edit.objective
        && (text.trim().is_empty() || text.len() > 10_000)
    {
        return Err(Error::Invalid("objective must contain 1-10000 bytes"));
    }
    if edit.token_budget.flatten().is_some_and(|cap| cap <= 0) {
        return Err(Error::Invalid("budget must be positive or null"));
    }
    for values in [&edit.success_criteria, &edit.forbidden]
        .into_iter()
        .flatten()
    {
        if values.len() > 100 || values.iter().any(|text| text.len() > 2000) {
            return Err(Error::Invalid("goal constraints exceed their limit"));
        }
    }
    if !matches!(edit.action, GoalEditAction::Edit | GoalEditAction::Override)
        && (edit.objective.is_some()
            || edit.token_budget.is_some()
            || edit.success_criteria.is_some()
            || edit.forbidden.is_some())
    {
        return Err(Error::Invalid("lifecycle actions cannot include edits"));
    }
    Ok(())
}
