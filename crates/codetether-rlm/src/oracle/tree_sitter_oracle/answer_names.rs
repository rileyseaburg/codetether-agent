use anyhow::Result;

pub(crate) fn functions(answer: &str) -> Result<Vec<String>> {
    names(answer, r"\bfn\s+(\w+)")
}

pub(crate) fn structs(answer: &str) -> Result<Vec<String>> {
    names(answer, r"\bstruct\s+(\w+)")
}

fn names(answer: &str, pattern: &str) -> Result<Vec<String>> {
    let re = regex::Regex::new(pattern)?;
    Ok(answer
        .lines()
        .filter_map(|line| re.captures(line))
        .filter_map(|cap| cap.get(1))
        .map(|name| name.as_str().to_string())
        .collect())
}
