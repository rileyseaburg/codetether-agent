//! Classify code line.

//! Line-type classification helpers.

pub(crate) fn is_code_line(line: &str) -> bool {
    let pats = [
        "function", "class ", "def ", "const ", "let ", "var ", "import ", "export ", "async ",
        "fn ", "impl ", "struct ", "enum ", "pub ", "use ", "mod ", "trait ",
    ];
    if pats.iter().any(|p| line.starts_with(p)) {
        return true;
    }
    if matches!(line, "{" | "}" | "(" | ")" | ";" | "{}" | "};") {
        return true;
    }
    line.starts_with("//")
        || line.starts_with('#')
        || line.starts_with('*')
        || line.starts_with("/*")
}
