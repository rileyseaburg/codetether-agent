use anyhow::Result;

use super::{ErrorPatternCounts, TreeSitterOracle};

const RESULT_QUERY: &str = r#"(generic_type type: (type_identifier) @name (#eq? @name "Result"))"#;
const TRY_QUERY: &str = r#"(try_expression)"#;
const MATCH_QUERY: &str = r#"(match_expression)"#;
const UNWRAP_QUERY: &str = r#"(call_expression function: (field_expression field: (field_identifier) @method (#eq? @method "unwrap")))"#;
const EXPECT_QUERY: &str = r#"(call_expression function: (field_expression field: (field_identifier) @method (#eq? @method "expect")))"#;

impl TreeSitterOracle {
    /// Count error handling patterns.
    pub fn count_error_patterns(&mut self) -> Result<ErrorPatternCounts> {
        Ok(ErrorPatternCounts {
            result_types: self.query(RESULT_QUERY)?.matches.len(),
            try_operators: self.query(TRY_QUERY)?.matches.len(),
            unwrap_calls: self.query(UNWRAP_QUERY)?.matches.len(),
            expect_calls: self.query(EXPECT_QUERY)?.matches.len(),
            match_expressions: self.query(MATCH_QUERY)?.matches.len(),
        })
    }
}
