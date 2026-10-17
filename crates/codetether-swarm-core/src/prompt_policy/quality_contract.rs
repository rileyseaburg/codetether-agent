//! Objective code-quality contract for mutating delegation prompts.

pub(crate) fn line_limit(agents_md: &str) -> Option<usize> {
    agents_md.lines().find_map(limit_from_line)
}

pub(super) fn render(read_only: bool, limit: Option<usize>) -> String {
    let Some(limit) = limit.filter(|_| !read_only) else {
        return String::new();
    };
    format!(
        "CODE QUALITY ACCEPTANCE CONTRACT:\n- Every changed source file MUST contain at most {limit} non-comment, nonblank lines.\n- Every changed module/file/function MUST have one clear responsibility.\n- Run the repository file-limit gate when available; otherwise count non-comment, nonblank lines statically.\n- Report each changed file with its measured code-line count.\n- Report every exception explicitly with its reason; do not silently grandfather new growth."
    )
}

fn limit_from_line(line: &str) -> Option<usize> {
    let lower = line.to_ascii_lowercase();
    if !(lower.contains("line") && (lower.contains("limit") || lower.contains("maximum"))) {
        return None;
    }
    lower
        .split(|character: char| !character.is_ascii_digit())
        .find_map(|part| part.parse::<usize>().ok())
        .filter(|limit| *limit > 0)
}

#[cfg(test)]
#[path = "quality_contract_tests.rs"]
mod tests;
