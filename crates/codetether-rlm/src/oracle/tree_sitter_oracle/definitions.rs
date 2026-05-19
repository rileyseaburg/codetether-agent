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
