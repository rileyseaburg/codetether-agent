/// Result of tree-sitter oracle verification.
#[derive(Debug, Clone, PartialEq)]
pub enum TreeSitterVerification {
    /// Answer matches AST truth exactly.
    ExactMatch,
    /// Answer matches but in different order.
    UnorderedMatch,
    /// Answer is a subset of the AST truth.
    SubsetMatch { claimed: usize, actual: usize },
    /// Answer contains incorrect claims.
    HasErrors { errors: Vec<String> },
    /// Answer is completely different.
    Mismatch,
    /// Could not parse or verify the answer.
    CannotVerify { reason: String },
}
