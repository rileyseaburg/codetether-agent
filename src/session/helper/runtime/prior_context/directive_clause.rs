//! Classification of one normalized user-text clause.

use super::directive::Directive;
use super::{actions, denial, permission, targets};

/// Classify one normalized clause as an access-policy directive.
pub(super) fn classify(text: &str) -> Option<Directive> {
    let target = targets::mentioned(text);
    if target && denial::matches(text) {
        return Some(if temporary(text) {
            Directive::DenyOnce
        } else {
            Directive::DenyPersistent
        });
    }
    let explicit_permission = super::permission_target::matches(text)
        && actions::mentioned(text)
        && permission::once(text);
    if super::source_preference::repository_only(text) && !explicit_permission {
        return Some(Directive::DenyPersistent);
    }
    if !explicit_permission {
        return None;
    }
    if !temporary(text) && permission::persistent(text) {
        return Some(Directive::AllowPersistent);
    }
    Some(Directive::AllowOnce)
}

fn temporary(text: &str) -> bool {
    ["this turn", "this request", "this time", "just once"]
        .iter()
        .any(|phrase| text.contains(phrase))
}
