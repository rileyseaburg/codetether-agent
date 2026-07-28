//! External resource registry for deterministic browser-agent pages.
//!
//! This module stores host-provided script, stylesheet, and image resources.
//! Page runtime code can apply registered text resources without performing
//! ambient network I/O, while image bytes remain available for inspection.

mod apply;
mod discover;
mod images;
mod inline;
mod page;
mod preload;
mod registry;
mod script;
mod script_arrow;
mod script_arrow_body;
mod script_arrow_params;
mod script_dom;
mod script_dynamic;
mod script_dynamic_ref;
mod script_export;
mod script_export_binding;
mod script_export_default;
mod script_export_names;
mod script_import;
mod script_import_alias;
mod script_import_bindings;
mod script_kind;
mod script_module;
mod script_module_refs;
mod script_namespace;
mod script_resolve;
mod source_map_page;
mod style;
mod types;
mod url;
mod url_norm;
mod validate;
mod validate_modules;
mod walk;

pub use registry::ResourceRegistry;
pub use types::{BrowserResource, ImageResourceMetadata, ResourceKind, ResourcePayload};

#[cfg(test)]
mod tests;
