use anyhow::{Result, anyhow};

/// Tree-sitter based oracle for validating structural queries.
pub struct TreeSitterOracle {
    pub(crate) source: String,
    pub(crate) tree: Option<tree_sitter::Tree>,
    pub(crate) parser: Option<tree_sitter::Parser>,
}

impl TreeSitterOracle {
    /// Create a new tree-sitter oracle for the given source.
    pub fn new(source: String) -> Self {
        Self {
            source,
            tree: None,
            parser: None,
        }
    }

    pub(crate) fn ensure_parser(&mut self) -> Result<()> {
        if self.parser.is_some() {
            return Ok(());
        }

        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_rust::LANGUAGE.into())?;
        self.parser = Some(parser);
        Ok(())
    }

    pub(crate) fn parse(&mut self) -> Result<&tree_sitter::Tree> {
        self.ensure_parser()?;
        if self.tree.is_none() {
            let parser = self
                .parser
                .as_mut()
                .ok_or_else(|| anyhow!("Parser not initialized"))?;
            self.tree = Some(parse_source(parser, &self.source)?);
        }
        self.tree
            .as_ref()
            .ok_or_else(|| anyhow!("Failed to retain parsed tree"))
    }
}

fn parse_source(parser: &mut tree_sitter::Parser, source: &str) -> Result<tree_sitter::Tree> {
    parser
        .parse(source, None)
        .ok_or_else(|| anyhow!("Failed to parse source"))
}
