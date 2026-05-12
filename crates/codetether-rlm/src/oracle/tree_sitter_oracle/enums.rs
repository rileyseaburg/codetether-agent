use anyhow::Result;

use super::{extract, EnumDefinition, TreeSitterOracle};

const ENUMS_QUERY: &str = r#"
(enum_item
    name: (type_identifier) @name
    body: (enum_variant_list)? @body)
"#;

impl TreeSitterOracle {
    /// Get all enum definitions in the source.
    pub fn get_enums(&mut self) -> Result<Vec<EnumDefinition>> {
        let result = self.query(ENUMS_QUERY)?;
        result.matches.into_iter().map(enum_definition).collect()
    }
}

fn enum_definition(m: super::AstMatch) -> Result<EnumDefinition> {
    let body = m.captures.get("body").cloned().unwrap_or_default();
    Ok(EnumDefinition {
        name: m.captures.get("name").cloned().unwrap_or_default(),
        variants: extract::enum_variants(&body)?,
        line: m.line,
    })
}
