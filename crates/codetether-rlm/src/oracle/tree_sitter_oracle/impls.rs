use anyhow::Result;

use super::{ImplDefinition, TreeSitterOracle};

const IMPLS_QUERY: &str = r#"
[
    (impl_item
        type: (type_identifier) @type
        trait: (type_identifier)? @trait
        body: (declaration_list)? @body)
    (impl_item
        for: (type_identifier) @for
        trait: (type_identifier) @trait
        body: (declaration_list)? @body)
]
"#;

impl TreeSitterOracle {
    /// Get all impl blocks in the source.
    pub fn get_impls(&mut self) -> Result<Vec<ImplDefinition>> {
        let result = self.query(IMPLS_QUERY)?;
        Ok(result.matches.into_iter().map(impl_definition).collect())
    }
}

fn impl_definition(m: super::AstMatch) -> ImplDefinition {
    let type_name = m
        .captures
        .get("type")
        .or_else(|| m.captures.get("for"))
        .cloned()
        .unwrap_or_default();
    let body = m.captures.get("body").cloned().unwrap_or_default();
    ImplDefinition {
        type_name,
        trait_name: m.captures.get("trait").cloned(),
        method_count: body.matches("fn ").count(),
        line: m.line,
    }
}
