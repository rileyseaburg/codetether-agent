use super::{TreeSitterOracle, TreeSitterVerification, answer_names, compare};

impl TreeSitterOracle {
    pub(crate) fn verify_functions(&mut self, answer: &str) -> TreeSitterVerification {
        let functions = match self.get_functions() {
            Ok(functions) => functions,
            Err(error) => return parse_failure("functions", error),
        };
        let actual = functions.into_iter().map(|f| f.name).collect();
        compare::compare_names(
            answer_names::functions(answer).unwrap_or_default(),
            actual,
            "Function",
        )
    }

    pub(crate) fn verify_structs(&mut self, answer: &str) -> TreeSitterVerification {
        let structs = match self.get_structs() {
            Ok(structs) => structs,
            Err(error) => return parse_failure("structs", error),
        };
        let actual = structs.into_iter().map(|s| s.name).collect();
        compare::compare_names(
            answer_names::structs(answer).unwrap_or_default(),
            actual,
            "Struct",
        )
    }
}

fn parse_failure(kind: &str, error: anyhow::Error) -> TreeSitterVerification {
    TreeSitterVerification::CannotVerify {
        reason: format!("Failed to parse {kind}: {error}"),
    }
}
