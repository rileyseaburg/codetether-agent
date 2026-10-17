//! Interpret line-numbered text before other answer formats.

use super::super::FinalAnswerFormat;
pub(super) fn parse(answer: &str) -> Option<FinalAnswerFormat> {
    // Try to parse as line-numbered matches
    let lines: Vec<&str> = answer.lines().collect();
    let mut numbered_matches = Vec::new();
    let mut all_valid = true;

    for line in &lines {
        // Pattern: "42:text" or "42: text" or "L42: text"
        let trimmed = line.trim();
        if let Some(colon_pos) = trimmed.find(':') {
            let num_part = trimmed[..colon_pos].trim().trim_start_matches('L').trim();
            if let Ok(line_num) = num_part.parse::<usize>() {
                let text_part = trimmed[colon_pos + 1..].trim().to_string();
                numbered_matches.push((line_num, text_part));
            } else {
                all_valid = false;
                break;
            }
        } else if !trimmed.is_empty() {
            // Non-empty line without line number
            all_valid = false;
            break;
        }
    }

    if all_valid && !numbered_matches.is_empty() {
        return Some(FinalAnswerFormat::LineNumberedMatches {
            matches: numbered_matches,
        });
    }

    None
}
