use anyhow::Result;

use super::{FunctionSignature, TreeSitterOracle};

const FUNCTIONS_QUERY: &str = r#"
(function_item
    name: (identifier) @name
    parameters: (parameters) @params
    return_type: (_)? @return_type)
"#;

impl TreeSitterOracle {
    /// Get all function signatures in the source.
    pub fn get_functions(&mut self) -> Result<Vec<FunctionSignature>> {
        let result = self.query(FUNCTIONS_QUERY)?;
        Ok(result.matches.into_iter().map(function_signature).collect())
    }
}

fn function_signature(m: super::AstMatch) -> FunctionSignature {
    FunctionSignature {
        name: m.captures.get("name").cloned().unwrap_or_default(),
        params: m.captures.get("params").cloned().unwrap_or_default(),
        return_type: m.captures.get("return_type").cloned(),
        line: m.line,
    }
}
