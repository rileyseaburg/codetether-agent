//! Stable ordered query catalog.

use super::QueryTemplate;
mod ast;
mod grep;
pub(super) const TEMPLATES: &[QueryTemplate] = &[
    grep::GREP_FIND_OCCURRENCES,
    grep::GREP_FIND_REGEX,
    grep::GREP_COUNT_OCCURRENCES,
    grep::GREP_FIND_FUNCTIONS,
    grep::GREP_FIND_STRUCTS,
    grep::GREP_FIND_ENUMS,
    grep::GREP_FIND_TRAITS,
    grep::GREP_FIND_IMPLS,
    grep::GREP_FIND_IMPORTS,
    grep::GREP_FIND_TESTS,
    grep::GREP_FIND_ERROR_HANDLING,
    grep::GREP_FIND_MACROS,
    grep::GREP_FIND_COMMENTS,
    grep::GREP_FIND_TODOS,
    grep::GREP_FIND_STRING_LITERALS,
    ast::AST_LIST_FUNCTIONS,
    ast::AST_LIST_STRUCTS,
    ast::AST_LIST_ENUMS,
    ast::AST_LIST_IMPLS,
    ast::AST_FUNCTION_SIGNATURES,
    ast::AST_PUB_FUNCTIONS,
    ast::AST_ASYNC_FUNCTIONS,
    ast::AST_STRUCT_FIELDS,
    ast::AST_ENUM_VARIANTS,
    ast::AST_TRAIT_IMPLS,
    ast::AST_ERROR_HANDLING_COUNT,
    ast::AST_METHOD_NAMES,
];
