use anyhow::Result;

use super::{extract, StructDefinition, TreeSitterOracle};

const STRUCTS_QUERY: &str = r#"
(struct_item
    name: (type_identifier) @name
    body: (field_declaration_list)? @body)
"#;

impl TreeSitterOracle {
    /// Get all struct definitions in the source.
    pub fn get_structs(&mut self) -> Result<Vec<StructDefinition>> {
        let result = self.query(STRUCTS_QUERY)?;
        result.matches.into_iter().map(struct_definition).collect()
    }
}

fn struct_definition(m: super::AstMatch) -> Result<StructDefinition> {
    let body = m.captures.get("body").cloned().unwrap_or_default();
    Ok(StructDefinition {
        name: m.captures.get("name").cloned().unwrap_or_default(),
        fields: extract::struct_fields(&body)?,
        line: m.line,
    })
}
