//! Namespace object generation for module resources.

use super::script_export_names;

pub(crate) fn name(url: &str) -> String {
    let mut out = "__tetherscript_module_".to_string();
    for byte in url.bytes() {
        if byte.is_ascii_alphanumeric() {
            out.push(byte as char);
        } else {
            out.push('_');
        }
    }
    out
}

pub(crate) fn binding(url: &str, source: &str) -> String {
    let entries = script_export_names::collect(source)
        .into_iter()
        .map(|name| {
            format!(
                "\"{}\": {}",
                name,
                super::script_export_binding::local(&name)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("let {} = {{{}}};\n", name(url), entries)
}
