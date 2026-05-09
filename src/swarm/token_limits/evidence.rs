pub fn extract_evidence(
    history: &[(&str, &str, bool)],
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut files = Vec::new();
    let mut tools = Vec::new();
    let mut errors = Vec::new();
    for (name, output, success) in history {
        tools.push((*name).to_string());
        if !success {
            errors.push(format!("{name}: {output}"));
        }
        if *name == "edit" || *name == "write" {
            files.extend(output.split_whitespace().filter_map(path_token));
        }
    }
    files.sort();
    files.dedup();
    tools.sort();
    tools.dedup();
    (files, tools, errors)
}

fn path_token(word: &str) -> Option<String> {
    let trimmed = word.trim_matches(|c| c == '"' || c == '\'' || c == ',');
    let looks_like_path = trimmed.contains('/')
        || [".rs", ".ts", ".tsx", ".py", ".go", ".md"]
            .iter()
            .any(|suffix| trimmed.ends_with(suffix));
    looks_like_path.then(|| trimmed.to_string())
}
