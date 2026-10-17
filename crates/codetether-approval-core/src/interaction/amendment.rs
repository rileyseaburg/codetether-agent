//! Command-prefix amendments proposed during interactive approval.

use serde::{Deserialize, Serialize};

/// Proposed exec-policy change for commands with a shared prefix.
///
/// # Examples
/// ```
/// use codetether_approval_core::interaction::ExecPolicyAmendment;
/// let amendment = ExecPolicyAmendment::new(vec!["git".into(), "status".into()]);
/// assert_eq!(amendment.prefix_string().as_deref(), Some("git status"));
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct ExecPolicyAmendment {
    /// Ordered command tokens forming the proposed prefix.
    pub command: Vec<String>,
}

impl ExecPolicyAmendment {
    /// Create a proposed command-prefix amendment.
    pub fn new(command: Vec<String>) -> Self {
        Self { command }
    }

    /// Return the command tokens that make up the reusable prefix.
    pub fn command(&self) -> &[String] {
        &self.command
    }

    /// Return trimmed, non-empty tokens joined by spaces, or no prefix.
    ///
    /// # Examples
    /// ```
    /// use codetether_approval_core::interaction::ExecPolicyAmendment;
    /// assert_eq!(ExecPolicyAmendment::new(vec![" ".into()]).prefix_string(), None);
    /// ```
    pub fn prefix_string(&self) -> Option<String> {
        let tokens = self
            .command
            .iter()
            .map(|token| token.trim())
            .filter(|token| !token.is_empty())
            .collect::<Vec<_>>();
        (!tokens.is_empty()).then(|| tokens.join(" "))
    }
}
