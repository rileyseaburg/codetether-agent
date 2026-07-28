//! Binding emission for rewritten module imports.

pub(crate) fn source(aliases: &[(String, String)]) -> String {
    aliases
        .iter()
        .filter(|(imported, local)| imported != local || imported == "default")
        .map(binding)
        .collect()
}

fn binding((imported, local): &(String, String)) -> String {
    format!(
        "let {} = {};\n",
        local,
        super::script_export_binding::local(imported)
    )
}
