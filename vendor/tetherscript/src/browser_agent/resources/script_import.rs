//! Static import extraction for deterministic module scripts.

use super::{script_export, script_import_alias};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ImportSpec {
    pub(crate) url: String,
    pub(crate) aliases: Vec<(String, String)>,
}

pub(crate) fn split(source: &str) -> (Vec<ImportSpec>, String) {
    let mut imports = Vec::new();
    let mut rest = source.trim_start();
    while is_static_import(rest) {
        let Some(end) = rest.find(';') else {
            break;
        };
        let statement = &rest[..=end];
        let Some(url) = quoted(statement) else {
            break;
        };
        imports.push(ImportSpec {
            url,
            aliases: script_import_alias::from_statement(statement),
        });
        rest = rest[end + 1..].trim_start();
    }
    (imports, script_export::strip(rest))
}

fn quoted(statement: &str) -> Option<String> {
    let start = statement.find(['"', '\''])?;
    let quote = statement.as_bytes()[start] as char;
    let tail = &statement[start + 1..];
    let end = tail.find(quote)?;
    Some(tail[..end].into())
}

fn is_static_import(source: &str) -> bool {
    source
        .strip_prefix("import")
        .and_then(|tail| tail.chars().next())
        .is_some_and(|ch| ch.is_whitespace() || ch == '{' || ch == '"' || ch == '\'')
}
