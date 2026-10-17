//! ast query definitions.

mod ast_list_functions;
pub(super) use ast_list_functions::VALUE as AST_LIST_FUNCTIONS;
mod ast_list_structs;
pub(super) use ast_list_structs::VALUE as AST_LIST_STRUCTS;
mod ast_list_enums;
pub(super) use ast_list_enums::VALUE as AST_LIST_ENUMS;
mod ast_list_impls;
pub(super) use ast_list_impls::VALUE as AST_LIST_IMPLS;
mod ast_function_signatures;
pub(super) use ast_function_signatures::VALUE as AST_FUNCTION_SIGNATURES;
mod ast_pub_functions;
pub(super) use ast_pub_functions::VALUE as AST_PUB_FUNCTIONS;
mod ast_async_functions;
pub(super) use ast_async_functions::VALUE as AST_ASYNC_FUNCTIONS;
mod ast_struct_fields;
pub(super) use ast_struct_fields::VALUE as AST_STRUCT_FIELDS;
mod ast_enum_variants;
pub(super) use ast_enum_variants::VALUE as AST_ENUM_VARIANTS;
mod ast_trait_impls;
pub(super) use ast_trait_impls::VALUE as AST_TRAIT_IMPLS;
mod ast_error_handling_count;
pub(super) use ast_error_handling_count::VALUE as AST_ERROR_HANDLING_COUNT;
mod ast_method_names;
pub(super) use ast_method_names::VALUE as AST_METHOD_NAMES;
