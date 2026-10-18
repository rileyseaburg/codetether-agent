//! Score shared directory suffixes.

pub(super) fn shared_path_suffix_score(left: &str, right: &str) -> usize {
    let left_parts: Vec<&str> = left.split('/').filter(|part| !part.is_empty()).collect();
    let right_parts: Vec<&str> = right.split('/').filter(|part| !part.is_empty()).collect();

    let mut score = 0usize;
    for (left_part, right_part) in left_parts.iter().rev().zip(right_parts.iter().rev()) {
        if left_part == right_part {
            score += 1;
        } else {
            break;
        }
    }

    score
}
