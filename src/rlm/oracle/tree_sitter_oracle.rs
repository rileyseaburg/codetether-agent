//! Tree-sitter oracle for structural AST verification.
//!
//! This oracle uses tree-sitter to parse source code and verify structural
//! queries about function signatures, struct fields, trait implementations, etc.
//!
//! # Supported Queries
//!
//! - Function signatures (name, args, return type)
//! - Struct/enum definitions and field listings
//! - Impl blocks and trait implementations
//! - Error handling patterns (Result, match arms, ? operator)
//!
//! # Features
//!
//! - Exposed as a verification oracle for FINAL() answers
//! - Also exposed as a new DSL command: `ast_query("(function_item)")`
//!
//! # Dependencies
//!
//! Requires `tree-sitter` and `tree-sitter-rust` crates.

use anyhow::{Result, anyhow};
use std::collections::HashMap;
use streaming_iterator::StreamingIterator;

use super::QueryType;

/// Tree-sitter based oracle for validating structural queries.
pub struct TreeSitterOracle {
    /// Source code content
    source: String,
    /// Parsed tree-sitter tree (lazy-initialized)
    tree: Option<tree_sitter::Tree>,
    /// Language parser
    parser: Option<tree_sitter::Parser>,
}

/// Result of tree-sitter AST query.
#[derive(Debug, Clone, PartialEq)]
pub struct AstQueryResult {
    /// Query type that was executed
    pub query_type: String,
    /// Matched nodes with their captures
    pub matches: Vec<AstMatch>,
}

/// A single match from an AST query.
#[derive(Debug, Clone, PartialEq)]
pub struct AstMatch {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column (1-indexed)
    pub column: usize,
    /// Captured nodes
    pub captures: HashMap<String, String>,
    /// Full text of the matched node
    pub text: String,
}

/// Result of tree-sitter oracle verification.
#[derive(Debug, Clone, PartialEq)]
pub enum TreeSitterVerification {
    /// Answer matches AST truth exactly.
    ExactMatch,
    /// Answer matches but in different order.
    UnorderedMatch,
    /// Answer is a subset (partial match).
    SubsetMatch {
        claimed: usize,
        actual: usize,
    },
    /// Answer contains incorrect claims.
    HasErrors {
        errors: Vec<String>,
    },
    /// Answer is completely different.
    Mismatch,
    /// Could not parse or verify.
    CannotVerify {
        reason: String,
    },
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

    /// Initialize the parser (lazy).
    fn ensure_parser(&mut self) -> Result<()> {
        if self.parser.is_some() {
            return Ok(());
        }

        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_rust::LANGUAGE.into())?;
        self.parser = Some(parser);
        Ok(())
    }

    /// Parse the source and return the tree.
    fn parse(&mut self) -> Result<&tree_sitter::Tree> {
        self.ensure_parser()?;
        
        if self.tree.is_none() {
            let parser = self.parser.as_mut().ok_or_else(|| anyhow!("Parser not initialized"))?;
            let tree = parser.parse(&self.source, None)
                .ok_or_else(|| anyhow!("Failed to parse source"))?;
            self.tree = Some(tree);
        }
        
        Ok(self.tree.as_ref().unwrap())
    }

    /// Execute a tree-sitter S-expression query.
    ///
    /// Example queries:
    /// - `(function_item name: (identifier) @name)`
    /// - `(struct_item name: (type_identifier) @name body: (field_declaration_list))`
    /// - `(impl_item trait: (type_identifier) @trait for: (type_identifier) @for)`
    pub fn query(&mut self, query_str: &str) -> Result<AstQueryResult> {
        self.parse()?;
        let tree = self.tree.as_ref().unwrap();
        let root = tree.root_node();
        
        let query = tree_sitter::Query::new(&tree_sitter_rust::LANGUAGE.into(), query_str)?;
        let mut cursor = tree_sitter::QueryCursor::new();
        
        let source_bytes = self.source.as_bytes();
        let mut results = Vec::new();
        
        let mut matches = cursor.matches(&query, root, source_bytes);
        while let Some(match_) = matches.next() {
            let mut captures = HashMap::new();
            let mut text = String::new();
            let mut line = 1;
            let mut column = 1;
            
            for capture in match_.captures {
                let node = capture.node;
                let capture_name = query.capture_names()[capture.index as usize].to_string();
                let capture_text = node.utf8_text(source_bytes)?.to_string();
                
                captures.insert(capture_name, capture_text.clone());
                
                if text.is_empty() {
                    text = capture_text;
                    line = node.start_position().row + 1;
                    column = node.start_position().column + 1;
                }
            }
            
            results.push(AstMatch {
                line,
                column,
                captures,
                text,
            });
        }
        
        Ok(AstQueryResult {
            query_type: query_str.to_string(),
            matches: results,
        })
    }

    /// Get all function signatures in the source.
    pub fn get_functions(&mut self) -> Result<Vec<FunctionSignature>> {
        let result = self.query(
            r#"
            (function_item
                name: (identifier) @name
                parameters: (parameters) @params
                return_type: (_)? @return_type)
            "#
        )?;
        
        let mut functions = Vec::new();
        for m in result.matches {
            let name = m.captures.get("name").cloned().unwrap_or_default();
            let params = m.captures.get("params").cloned().unwrap_or_default();
            let return_type = m.captures.get("return_type").cloned();
            
            functions.push(FunctionSignature {
                name,
                params,
                return_type,
                line: m.line,
            });
        }
        
        Ok(functions)
    }

    /// Get all struct definitions in the source.
    pub fn get_structs(&mut self) -> Result<Vec<StructDefinition>> {
        let result = self.query(
            r#"
            (struct_item
                name: (type_identifier) @name
                body: (field_declaration_list)? @body)
            "#
        )?;
        
        let mut structs = Vec::new();
        for m in result.matches {
            let name = m.captures.get("name").cloned().unwrap_or_default();
            let body = m.captures.get("body").cloned().unwrap_or_default();
            
            // Extract fields from body
            let fields = self.extract_struct_fields(&body)?;
            
            structs.push(StructDefinition {
                name,
                fields,
                line: m.line,
            });
        }
        
        Ok(structs)
    }

    /// Extract field names from a struct body.
    fn extract_struct_fields(&self, body: &str) -> Result<Vec<String>> {
        let mut fields = Vec::new();
        
        // Simple regex-based extraction (faster than re-parsing)
        let re = regex::Regex::new(r"(?:pub\s+)?(\w+)\s*:")?;
        for cap in re.captures_iter(body) {
            if let Some(name) = cap.get(1) {
                fields.push(name.as_str().to_string());
            }
        }
        
        Ok(fields)
    }

    /// Get all enum definitions in the source.
    pub fn get_enums(&mut self) -> Result<Vec<EnumDefinition>> {
        let result = self.query(
            r#"
            (enum_item
                name: (type_identifier) @name
                body: (enum_variant_list)? @body)
            "#
        )?;
        
        let mut enums = Vec::new();
        for m in result.matches {
            let name = m.captures.get("name").cloned().unwrap_or_default();
            let body = m.captures.get("body").cloned().unwrap_or_default();
            
            // Extract variants from body
            let variants = self.extract_enum_variants(&body)?;
            
            enums.push(EnumDefinition {
                name,
                variants,
                line: m.line,
            });
        }
        
        Ok(enums)
    }

    /// Extract variant names from an enum body.
    fn extract_enum_variants(&self, body: &str) -> Result<Vec<String>> {
        let mut variants = Vec::new();
        
        let re = regex::Regex::new(r"(\w+)\s*(?:,|=|\{|\()")?;
        for cap in re.captures_iter(body) {
            if let Some(name) = cap.get(1) {
                let name_str = name.as_str();
                // Skip keywords
                if !["pub", "fn", "struct", "enum", "impl", "trait"].contains(&name_str) {
                    variants.push(name_str.to_string());
                }
            }
        }
        
        Ok(variants)
    }

    /// Get all impl blocks in the source.
    pub fn get_impls(&mut self) -> Result<Vec<ImplDefinition>> {
        let result = self.query(
            r#"
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
            "#
        )?;
        
        let mut impls = Vec::new();
        for m in result.matches {
            let type_name = m.captures.get("type")
                .or_else(|| m.captures.get("for"))
                .cloned()
                .unwrap_or_default();
            let trait_name = m.captures.get("trait").cloned();
            let body = m.captures.get("body").cloned().unwrap_or_default();
            
            impls.push(ImplDefinition {
                type_name,
                trait_name,
                method_count: body.matches("fn ").count(),
                line: m.line,
            });
        }
        
        Ok(impls)
    }

    /// Count error handling patterns.
    pub fn count_error_patterns(&mut self) -> Result<ErrorPatternCounts> {
        // Count Result<T> types
        let result_types = self.query(r#"(generic_type type: (type_identifier) @name (#eq? @name "Result"))"#)?;
        
        // Count ? operators
        let try_operators = self.query(r#"(try_expression)"#)?;
        
        // Count .unwrap() calls
        let unwrap_calls = self.query(r#"(call_expression function: (field_expression field: (field_identifier) @method (#eq? @method "unwrap")))"#)?;
        
        // Count .expect() calls
        let expect_calls = self.query(r#"(call_expression function: (field_expression field: (field_identifier) @method (#eq? @method "expect")))"#)?;
        
        // Count match expressions
        let match_exprs = self.query(r#"(match_expression)"#)?;
        
        Ok(ErrorPatternCounts {
            result_types: result_types.matches.len(),
            try_operators: try_operators.matches.len(),
            unwrap_calls: unwrap_calls.matches.len(),
            expect_calls: expect_calls.matches.len(),
            match_expressions: match_exprs.matches.len(),
        })
    }

    /// Verify a FINAL() answer against AST truth.
    pub fn verify(&mut self, answer: &str, query: &str) -> TreeSitterVerification {
        let query_type = Self::classify_query(query);
        
        match query_type {
            QueryType::Structural => {
                // Try to match against different structural queries
                if query.to_lowercase().contains("function") {
                    self.verify_functions(answer)
                } else if query.to_lowercase().contains("struct") {
                    self.verify_structs(answer)
                } else if query.to_lowercase().contains("enum") {
                    self.verify_enums(answer)
                } else if query.to_lowercase().contains("impl") {
                    self.verify_impls(answer)
                } else {
                    TreeSitterVerification::CannotVerify {
                        reason: "Unknown structural query type".to_string(),
                    }
                }
            }
            _ => TreeSitterVerification::CannotVerify {
                reason: "Not a structural query".to_string(),
            }
        }
    }

    /// Classify query type for tree-sitter routing.
    fn classify_query(query: &str) -> QueryType {
        let lower = query.to_lowercase();
        
        if lower.contains("signature")
            || lower.contains("parameters")
            || lower.contains("return type")
            || lower.contains("fields of")
            || lower.contains("what fields")
            || lower.contains("struct definition")
            || lower.contains("enum variants")
            || lower.contains("implements")
            || lower.contains("methods")
        {
            return QueryType::Structural;
        }
        
        QueryType::Semantic
    }

    fn verify_functions(&mut self, answer: &str) -> TreeSitterVerification {
        let functions = match self.get_functions() {
            Ok(f) => f,
            Err(e) => return TreeSitterVerification::CannotVerify {
                reason: format!("Failed to parse functions: {}", e),
            },
        };
        
        // Parse answer to extract claimed function names
        let claimed_names: Vec<String> = answer
            .lines()
            .filter_map(|line| {
                // Try to extract function name from line
                let re = regex::Regex::new(r"\bfn\s+(\w+)").ok()?;
                re.captures(line)
                    .and_then(|cap| cap.get(1))
                    .map(|m| m.as_str().to_string())
            })
            .collect();
        
        if claimed_names.is_empty() {
            return TreeSitterVerification::CannotVerify {
                reason: "Could not extract function names from answer".to_string(),
            };
        }
        
        let actual_names: Vec<String> = functions.iter().map(|f| f.name.clone()).collect();
        let claimed_set: std::collections::HashSet<_> = claimed_names.iter().cloned().collect();
        let actual_set: std::collections::HashSet<_> = actual_names.iter().cloned().collect();
        
        if claimed_set == actual_set {
            TreeSitterVerification::ExactMatch
        } else if claimed_set.is_subset(&actual_set) {
            TreeSitterVerification::SubsetMatch {
                claimed: claimed_names.len(),
                actual: actual_names.len(),
            }
        } else {
            let errors = claimed_names
                .iter()
                .filter(|name| !actual_set.contains(*name))
                .map(|name| format!("Function '{}' not found", name))
                .collect();
            TreeSitterVerification::HasErrors { errors }
        }
    }

    fn verify_structs(&mut self, answer: &str) -> TreeSitterVerification {
        let structs = match self.get_structs() {
            Ok(s) => s,
            Err(e) => return TreeSitterVerification::CannotVerify {
                reason: format!("Failed to parse structs: {}", e),
            },
        };
        
        // Similar logic to verify_functions
        let claimed_names: Vec<String> = answer
            .lines()
            .filter_map(|line| {
                let re = regex::Regex::new(r"\bstruct\s+(\w+)").ok()?;
                re.captures(line)
                    .and_then(|cap| cap.get(1))
                    .map(|m| m.as_str().to_string())
            })
            .collect();
        
        if claimed_names.is_empty() {
            return TreeSitterVerification::CannotVerify {
                reason: "Could not extract struct names from answer".to_string(),
            };
        }
        
        let actual_names: Vec<String> = structs.iter().map(|s| s.name.clone()).collect();
        let claimed_set: std::collections::HashSet<_> = claimed_names.iter().cloned().collect();
        let actual_set: std::collections::HashSet<_> = actual_names.iter().cloned().collect();
        
        if claimed_set == actual_set {
            TreeSitterVerification::ExactMatch
        } else if claimed_set.is_subset(&actual_set) {
            TreeSitterVerification::SubsetMatch {
                claimed: claimed_names.len(),
                actual: actual_names.len(),
            }
        } else {
            let errors = claimed_names
                .iter()
                .filter(|name| !actual_set.contains(*name))
                .map(|name| format!("Struct '{}' not found", name))
                .collect();
            TreeSitterVerification::HasErrors { errors }
        }
    }

    fn verify_enums(&mut self, _answer: &str) -> TreeSitterVerification {
        // Similar pattern
        TreeSitterVerification::CannotVerify {
            reason: "Enum verification not yet implemented".to_string(),
        }
    }

    fn verify_impls(&mut self, _answer: &str) -> TreeSitterVerification {
        // Similar pattern
        TreeSitterVerification::CannotVerify {
            reason: "Impl verification not yet implemented".to_string(),
        }
    }
}

/// Function signature information.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub name: String,
    pub params: String,
    pub return_type: Option<String>,
    pub line: usize,
}

/// Struct definition information.
#[derive(Debug, Clone, PartialEq)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<String>,
    pub line: usize,
}

/// Enum definition information.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDefinition {
    pub name: String,
    pub variants: Vec<String>,
    pub line: usize,
}

/// Impl block information.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplDefinition {
    pub type_name: String,
    pub trait_name: Option<String>,
    pub method_count: usize,
    pub line: usize,
}

/// Counts of error handling patterns.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorPatternCounts {
    pub result_types: usize,
    pub try_operators: usize,
    pub unwrap_calls: usize,
    pub expect_calls: usize,
    pub match_expressions: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_rust_code() -> String {
        r#"
use anyhow::Result;

pub struct Config {
    pub debug: bool,
    pub timeout: u64,
}

impl Config {
    pub fn new() -> Self {
        Self { debug: false, timeout: 30 }
    }
    
    pub fn with_debug(mut self) -> Self {
        self.debug = true;
        self
    }
}

pub async fn process(input: &str) -> Result<String> {
    let data = parse(input)?;
    Ok(data.to_uppercase())
}

fn parse(input: &str) -> Result<String> {
    if input.is_empty() {
        return Err(anyhow!("empty input"));
    }
    Ok(input.to_string())
}

enum Status {
    Active,
    Inactive,
    Pending,
}
"#.to_string()
    }

    #[test]
    fn get_functions_finds_all() {
        let mut oracle = TreeSitterOracle::new(sample_rust_code());
        let functions = oracle.get_functions().unwrap();
        assert!(functions.len() >= 3);
        
        let names: Vec<&str> = functions.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"new"));
        assert!(names.contains(&"with_debug"));
        assert!(names.contains(&"process"));
        assert!(names.contains(&"parse"));
    }

    #[test]
    fn get_structs_finds_all() {
        let mut oracle = TreeSitterOracle::new(sample_rust_code());
        let structs = oracle.get_structs().unwrap();
        assert!(structs.len() >= 1);
        
        let config_struct = structs.iter().find(|s| s.name == "Config").unwrap();
        assert!(config_struct.fields.contains(&"debug".to_string()));
        assert!(config_struct.fields.contains(&"timeout".to_string()));
    }

    #[test]
    fn get_enums_finds_all() {
        let mut oracle = TreeSitterOracle::new(sample_rust_code());
        let enums = oracle.get_enums().unwrap();
        assert!(enums.len() >= 1);
        
        let status_enum = enums.iter().find(|e| e.name == "Status").unwrap();
        assert!(status_enum.variants.contains(&"Active".to_string()));
        assert!(status_enum.variants.contains(&"Inactive".to_string()));
    }

    #[test]
    fn count_error_patterns() {
        let mut oracle = TreeSitterOracle::new(sample_rust_code());
        let counts = oracle.count_error_patterns().unwrap();
        
        assert!(counts.result_types >= 2); // At least 2 Result<T>
        assert!(counts.try_operators >= 1); // At least 1 ? operator
    }
}
