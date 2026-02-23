//! Oracle-friendly query templates for high-golden-rate trace generation.
//!
//! These templates are designed to produce verifiable queries that the oracles
//! can deterministically validate. Each template specifies:
//! - The query text
//! - The expected oracle kind (grep, ast, or semantic)
//! - Example parameters
//!
//! # Usage
//!
//! ```ignore
//! use codetether_agent::rlm::oracle::templates::{QueryTemplate, TemplateKind};
//!
//! // Generate a grep template query
//! let template = QueryTemplate::find_all_pattern("async fn", "src/main.rs");
//! println!("{}", template.render()); // "Find all occurrences of async fn in src/main.rs"
//!
//! // Get all templates of a certain kind
//! let grep_templates = QueryTemplate::all()
//!     .filter(|t| t.kind == TemplateKind::Grep);
//! ```

use serde::{Deserialize, Serialize};

/// The kind of oracle this template is designed for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateKind {
    /// Grep/pattern-match oracle
    Grep,
    /// Tree-sitter/AST oracle
    Ast,
    /// Semantic (unverifiable) - included for completeness
    Semantic,
}

impl TemplateKind {
    /// Convert to the FinalPayload kind string.
    pub fn as_payload_kind(&self) -> &'static str {
        match self {
            TemplateKind::Grep => "grep",
            TemplateKind::Ast => "ast",
            TemplateKind::Semantic => "semantic",
        }
    }
}

/// A query template for oracle-friendly trace generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTemplate {
    /// Unique template identifier
    pub id: &'static str,
    /// Human-readable template description
    pub description: &'static str,
    /// The template string with {PLACEHOLDERS}
    pub template: &'static str,
    /// Parameters this template expects
    #[serde(skip)]
    pub params: &'static [Param],
    /// Expected oracle kind
    pub kind: TemplateKind,
    /// Example parameter values
    #[serde(skip)]
    pub example: &'static [&'static str],
}

/// A parameter in a query template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    /// Parameter name (used as placeholder)
    pub name: &'static str,
    /// Description of what value to provide
    pub description: &'static str,
    /// Example value
    pub example: &'static str,
}

impl QueryTemplate {
    /// Render this template with the given parameter values.
    pub fn render(&self, values: &[&str]) -> String {
        let mut result = self.template.to_string();
        
        for (i, value) in values.iter().enumerate() {
            if i < self.params.len() {
                let placeholder = format!("{{{}}}", self.params[i].name);
                result = result.replace(&placeholder, value);
            }
        }
        
        result
    }

    /// Get all defined templates.
    pub fn all() -> &'static [QueryTemplate] {
        TEMPLATES
    }

    /// Get templates by kind.
    pub fn by_kind(kind: TemplateKind) -> impl Iterator<Item = &'static QueryTemplate> {
        TEMPLATES.iter().filter(move |t| t.kind == kind)
    }

    /// Find a template by ID.
    pub fn find(id: &str) -> Option<&'static QueryTemplate> {
        TEMPLATES.iter().find(|t| t.id == id)
    }

    /// Generate a random example query (for testing).
    #[allow(dead_code)]
    pub fn random_example(&self) -> String {
        self.render(self.example)
    }
}

// Define all templates
const TEMPLATES: &[QueryTemplate] = &[
    // ==================== GREP TEMPLATES ====================
    QueryTemplate {
        id: "grep_find_occurrences",
        description: "Find all occurrences of a string pattern in a file",
        template: "Find all occurrences of {PATTERN} in {FILE}",
        params: &[
            Param {
                name: "PATTERN",
                description: "String or regex pattern to search for",
                example: "async fn",
            },
            Param {
                name: "FILE",
                description: "File path to search in",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["async fn", "src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_regex",
        description: "Find all lines matching a regex pattern in a file",
        template: "Find all lines matching {REGEX} in {FILE}",
        params: &[
            Param {
                name: "REGEX",
                description: "Regular expression pattern",
                example: r"\bpub\s+fn\b",
            },
            Param {
                name: "FILE",
                description: "File path to search in",
                example: "src/lib.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &[r"\bpub\s+fn\b", "src/lib.rs"],
    },
    QueryTemplate {
        id: "grep_count_occurrences",
        description: "Count occurrences of a pattern in a file",
        template: "Count occurrences of {PATTERN} in {FILE}",
        params: &[
            Param {
                name: "PATTERN",
                description: "Pattern to count",
                example: "TODO",
            },
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["TODO", "src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_functions",
        description: "Find all function definitions matching a pattern",
        template: "Find all {VISIBILITY} functions matching {PATTERN} in {FILE}",
        params: &[
            Param {
                name: "VISIBILITY",
                description: "public, private, or all (leave empty)",
                example: "public",
            },
            Param {
                name: "PATTERN",
                description: "Pattern in function definition",
                example: "async fn",
            },
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["public", "async fn", "src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_structs",
        description: "Find all struct definitions",
        template: "Find all structs in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_enums",
        description: "Find all enum definitions",
        template: "Find all enums in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_traits",
        description: "Find all trait definitions",
        template: "Find all traits in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_impls",
        description: "Find all impl blocks",
        template: "Find all impl blocks in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_imports",
        description: "Find all use/import statements",
        template: "Find all imports in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_tests",
        description: "Find all test functions",
        template: "Find all test functions in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_error_handling",
        description: "Find error handling patterns (Result, ?, unwrap)",
        template: "Find all error handling patterns in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_macros",
        description: "Find macro invocations",
        template: "Find all macro calls in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_comments",
        description: "Find comments in code",
        template: "Find all comments in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_todos",
        description: "Find TODO/FIXME comments",
        template: "Find all TODO comments in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "grep_find_string_literals",
        description: "Find string literals",
        template: "Find all string literals in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Grep,
        example: &["src/main.rs"],
    },

    // ==================== AST TEMPLATES ====================
    QueryTemplate {
        id: "ast_list_functions",
        description: "List all function names in a file",
        template: "List all function names in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "ast_list_structs",
        description: "List all structs and their fields",
        template: "List all structs and fields in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "ast_list_enums",
        description: "List all enums and their variants",
        template: "List all enums and variants in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "ast_list_impls",
        description: "List all impl blocks for a type",
        template: "List all impl blocks for {TYPE} in {FILE}",
        params: &[
            Param {
                name: "TYPE",
                description: "Type name to find impls for",
                example: "MyStruct",
            },
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["MyStruct", "src/main.rs"],
    },
    QueryTemplate {
        id: "ast_function_signatures",
        description: "Get function signatures (name, params, return type)",
        template: "List all function signatures in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "ast_pub_functions",
        description: "List public function signatures",
        template: "List all public function signatures in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "ast_async_functions",
        description: "List async function signatures",
        template: "List all async function signatures in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "ast_struct_fields",
        description: "Get struct field names and types",
        template: "What fields does {STRUCT} have in {FILE}?",
        params: &[
            Param {
                name: "STRUCT",
                description: "Struct name",
                example: "Config",
            },
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["Config", "src/main.rs"],
    },
    QueryTemplate {
        id: "ast_enum_variants",
        description: "Get enum variant names",
        template: "What variants does {ENUM} have in {FILE}?",
        params: &[
            Param {
                name: "ENUM",
                description: "Enum name",
                example: "Status",
            },
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["Status", "src/main.rs"],
    },
    QueryTemplate {
        id: "ast_trait_impls",
        description: "Find trait implementations",
        template: "What traits does {TYPE} implement in {FILE}?",
        params: &[
            Param {
                name: "TYPE",
                description: "Type name",
                example: "MyStruct",
            },
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["MyStruct", "src/main.rs"],
    },
    QueryTemplate {
        id: "ast_error_handling_count",
        description: "Count error handling patterns",
        template: "Count the error handling patterns (Result, ?, unwrap, expect) in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["src/main.rs"],
    },
    QueryTemplate {
        id: "ast_method_names",
        description: "List all method names in impl blocks",
        template: "List all methods defined in impl blocks in {FILE}",
        params: &[
            Param {
                name: "FILE",
                description: "File path",
                example: "src/main.rs",
            },
        ],
        kind: TemplateKind::Ast,
        example: &["src/main.rs"],
    },
];

/// A pre-generated query ready for use.
#[derive(Debug, Clone)]
pub struct GeneratedQuery {
    /// The rendered query text
    pub query: String,
    /// Template ID used
    pub template_id: &'static str,
    /// Expected oracle kind
    pub kind: TemplateKind,
    /// File the query targets
    pub file: String,
}

impl GeneratedQuery {
    /// Render a template with custom values.
    pub fn render(template: &'static QueryTemplate, values: &[&str]) -> Self {
        let file = values.last().unwrap_or(&"").to_string();
        
        Self {
            query: template.render(values),
            template_id: template.id,
            kind: template.kind,
            file,
        }
    }
}

/// Iterator over templates of a specific kind.
pub struct TemplateIter {
    index: usize,
    kind: Option<TemplateKind>,
}

impl Iterator for TemplateIter {
    type Item = &'static QueryTemplate;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < TEMPLATES.len() {
            let template = &TEMPLATES[self.index];
            self.index += 1;
            
            if let Some(kind) = self.kind {
                if template.kind == kind {
                    return Some(template);
                }
            } else {
                return Some(template);
            }
        }
        None
    }
}

impl QueryTemplate {
    /// Iterate over all templates.
    pub fn iter() -> TemplateIter {
        TemplateIter { index: 0, kind: None }
    }

    /// Iterate over templates of a specific kind.
    pub fn iter_kind(kind: TemplateKind) -> TemplateIter {
        TemplateIter { index: 0, kind: Some(kind) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_simple_template() {
        let template = QueryTemplate::find("grep_find_occurrences").unwrap();
        let rendered = template.render(&["fn", "src/main.rs"]);
        assert_eq!(rendered, "Find all occurrences of fn in src/main.rs");
    }

    #[test]
    fn find_template_by_id() {
        let template = QueryTemplate::find("ast_list_functions").unwrap();
        assert_eq!(template.kind, TemplateKind::Ast);
    }

    #[test]
    fn filter_templates_by_kind() {
        let grep_count = QueryTemplate::iter_kind(TemplateKind::Grep).count();
        let ast_count = QueryTemplate::iter_kind(TemplateKind::Ast).count();
        
        assert!(grep_count > 0);
        assert!(ast_count > 0);
    }

    #[test]
    fn template_has_examples() {
        for template in QueryTemplate::all() {
            assert!(!template.example.is_empty(), "Template {} has no examples", template.id);
        }
    }

    #[test]
    fn generated_query_has_fields() {
        let template = QueryTemplate::find("grep_find_occurrences").unwrap();
        let r#gen = GeneratedQuery::render(template, &["async fn", "src/lib.rs"]);
        
        assert_eq!(r#gen.query, "Find all occurrences of async fn in src/lib.rs");
        assert_eq!(r#gen.template_id, "grep_find_occurrences");
        assert_eq!(r#gen.kind, TemplateKind::Grep);
        assert_eq!(r#gen.file, "src/lib.rs");
    }

    #[test]
    fn template_kind_to_payload_kind() {
        assert_eq!(TemplateKind::Grep.as_payload_kind(), "grep");
        assert_eq!(TemplateKind::Ast.as_payload_kind(), "ast");
        assert_eq!(TemplateKind::Semantic.as_payload_kind(), "semantic");
    }
}
