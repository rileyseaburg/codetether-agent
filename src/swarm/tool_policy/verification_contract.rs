//! Verification requirements inferred from delegated task instructions.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VerificationContract {
    None,
    FocusedRequired,
    StaticOnly,
    Waived,
}

impl VerificationContract {
    pub(crate) fn from_instruction(instruction: &str) -> Self {
        let lower = instruction.to_ascii_lowercase();
        if super::verification_language::waived(&lower) {
            Self::Waived
        } else if super::verification_language::preserves_behavior(&lower)
            && super::verification_language::prohibits_verification(&lower)
        {
            Self::StaticOnly
        } else if super::verification_language::preserves_behavior(&lower) {
            Self::FocusedRequired
        } else {
            Self::None
        }
    }

    pub(super) fn prompt(self) -> &'static str {
        match self {
            Self::None => "",
            Self::FocusedRequired => {
                "VERIFICATION CONTRACT: A behavioral-preservation claim requires focused verification. Run focused tests or explicitly report the claim as not verified."
            }
            Self::StaticOnly => {
                "VERIFICATION CONFLICT: Behavioral preservation was requested while execution verification was prohibited. Perform source inspection only and label the result static/local; do not claim behavioral equivalence or full preservation."
            }
            Self::Waived => {
                "VERIFICATION WAIVER: Verification was explicitly waived. Report behavioral preservation as not-run rather than proven."
            }
        }
    }
}

#[cfg(test)]
#[path = "verification_contract_tests.rs"]
mod tests;
