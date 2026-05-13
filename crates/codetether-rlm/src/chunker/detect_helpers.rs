//! Line-type classification helpers.

pub(crate) fn is_code_line(line: &str) -> bool {
    let pats = ["function", "class ", "def ", "const ", "let ", "var ", "import ", "export ", "async ", "fn ", "impl ", "struct ", "enum ", "pub ", "use ", "mod ", "trait "];
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

pub(crate) fn is_log_line(line: &str) -> bool {
    if line.len() >= 10 && line.chars().take(4).all(|c| c.is_ascii_digit()) && line.chars().nth(4) == Some('-') {
        return true;
    }
    if line.starts_with('[') && line.len() > 5 && line.chars().nth(1).is_some_and(|c| c.is_ascii_digit()) {
        return true;
    }
    let lvls = ["INFO", "DEBUG", "WARN", "ERROR", "FATAL", "TRACE"];
    lvls.iter().any(|l| line.starts_with(l) || line.contains(&format!(" {} ", l)))
}

pub(crate) fn is_conversation_line(line: &str) -> bool {
    let p = [
        "[User]:", "[Assistant]:", "[Human]:", "[AI]:", "User:", "Assistant:", "Human:", "AI:",
        "[Tool ", "<user>", "<assistant>", "<system>",
    ];
    p.iter().any(|p| line.starts_with(p))
}

pub(crate) fn is_document_line(line: &str) -> bool {
    if line.starts_with('#') && line.chars().nth(1).is_some_and(|c| c == ' ' || c == '#') {
        return true;
    }
    if line.starts_with("**") && line.contains("**") {
        return true;
    }
    if line.starts_with("> ") {
        return true;
    }
    if line.starts_with("- ") && line.len() > 3 {
        return true;
    }
    line.len() > 80 && !line.ends_with('{') && !line.ends_with(';')
        && !line.ends_with('(') && !line.ends_with(')') && !line.ends_with('=')
}
