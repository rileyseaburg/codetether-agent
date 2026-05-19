use crate::oracle::QueryType;

use super::{TreeSitterOracle, TreeSitterVerification};

impl TreeSitterOracle {
    /// Verify a FINAL() answer against AST truth.
    pub fn verify(&mut self, answer: &str, query: &str) -> TreeSitterVerification {
        if Self::classify_query(query) != QueryType::Structural {
            return TreeSitterVerification::CannotVerify {
                reason: "Not a structural query".to_string(),
            };
        }
        self.verify_structural(answer, query)
    }

    fn verify_structural(&mut self, answer: &str, query: &str) -> TreeSitterVerification {
        let lower = query.to_lowercase();
        if lower.contains("function") {
            self.verify_functions(answer)
        } else if lower.contains("struct") {
            self.verify_structs(answer)
        } else if lower.contains("enum") {
            unimplemented_verification("Enum")
        } else if lower.contains("impl") {
            unimplemented_verification("Impl")
        } else {
            TreeSitterVerification::CannotVerify {
                reason: "Unknown structural query type".to_string(),
            }
        }
    }
}

fn unimplemented_verification(kind: &str) -> TreeSitterVerification {
    TreeSitterVerification::CannotVerify {
        reason: format!("{kind} verification not yet implemented"),
    }
}
