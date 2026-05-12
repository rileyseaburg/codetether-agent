use std::collections::HashSet;

use super::TreeSitterVerification;

pub(crate) fn compare_names(
    claimed: Vec<String>,
    actual: Vec<String>,
    label: &str,
) -> TreeSitterVerification {
    if claimed.is_empty() {
        return TreeSitterVerification::CannotVerify {
            reason: format!("Could not extract {label} names from answer"),
        };
    }

    let claimed_set: HashSet<_> = claimed.iter().cloned().collect();
    let actual_set: HashSet<_> = actual.iter().cloned().collect();
    if claimed_set == actual_set {
        TreeSitterVerification::ExactMatch
    } else if claimed_set.is_subset(&actual_set) {
        TreeSitterVerification::SubsetMatch {
            claimed: claimed.len(),
            actual: actual.len(),
        }
    } else {
        TreeSitterVerification::HasErrors {
            errors: unexpected_errors(&claimed, &actual_set, label),
        }
    }
}

fn unexpected_errors(claimed: &[String], actual: &HashSet<String>, label: &str) -> Vec<String> {
    claimed
        .iter()
        .filter(|name| !actual.contains(*name))
        .map(|name| format!("{label} '{name}' not found"))
        .collect()
}
