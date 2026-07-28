//! Similarity scoring helpers.

pub fn clean(s: &str) -> String {
    s.trim().to_ascii_lowercase()
}

pub fn opt_eq(a: &Option<String>, b: &Option<String>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => clean(a) == clean(b),
        _ => false,
    }
}

pub fn text_eq(a: &Option<String>, b: &Option<String>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => {
            let a = clean(a);
            let b = clean(b);
            !a.is_empty() && (a == b || a.contains(&b) || b.contains(&a))
        }
        _ => false,
    }
}

pub fn class_overlap(a: &[String], b: &[String]) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let hit = a
        .iter()
        .filter(|x| b.iter().any(|y| clean(x) == clean(y)))
        .count();
    hit as f64 / a.len().max(b.len()) as f64
}
