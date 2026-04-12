//! String matching strategies for advanced edit tool

/// Levenshtein distance for fuzzy matching
pub fn levenshtein(a: &str, b: &str) -> usize {
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut matrix = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for i in 0..=a.len() {
        matrix[i][0] = i;
    }
    for j in 0..=b.len() {
        matrix[0][j] = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    matrix[a.len()][b.len()]
}

pub type Replacer = fn(&str, &str) -> Vec<String>;

/// Simple exact match
pub fn simple_replacer(content: &str, find: &str) -> Vec<String> {
    if content.contains(find) {
        vec![find.to_string()]
    } else {
        vec![]
    }
}

/// Match with trimmed lines
pub fn line_trimmed_replacer(content: &str, find: &str) -> Vec<String> {
    let orig_lines: Vec<&str> = content.lines().collect();
    let mut search_lines: Vec<&str> = find.lines().collect();
    if search_lines.last() == Some(&"") {
        search_lines.pop();
    }
    let mut results = vec![];
    for i in 0..=orig_lines.len().saturating_sub(search_lines.len()) {
        let mut matches = true;
        for j in 0..search_lines.len() {
            if orig_lines.get(i + j).map(|l| l.trim()) != Some(search_lines[j].trim()) {
                matches = false;
                break;
            }
        }
        if matches {
            let matched: Vec<&str> = orig_lines[i..i + search_lines.len()].to_vec();
            results.push(matched.join("\n"));
        }
    }
    results
}
