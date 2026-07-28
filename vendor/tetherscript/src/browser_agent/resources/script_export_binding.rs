//! Generated local names for rewritten module exports.

const DEFAULT_LOCAL: &str = "__tetherscript_default_export";

pub(crate) fn local(exported: &str) -> String {
    if exported.trim() == "default" {
        DEFAULT_LOCAL.into()
    } else {
        exported.trim().into()
    }
}
