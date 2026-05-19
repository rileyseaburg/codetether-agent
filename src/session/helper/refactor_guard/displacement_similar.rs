use super::types::{GuardReport, GuardViolation};

pub fn apply(report: &GuardReport, out: &mut Vec<GuardViolation>) {
    for old in report
        .files
        .iter()
        .filter(|file| file.old_code_lines.unwrap_or(0) > file.limit)
    {
        let Some(old_text) = old.old_text.as_deref() else {
            continue;
        };
        if old.new_code_lines + 40 >= old.old_code_lines.unwrap_or(0) {
            continue;
        }
        for new in report
            .files
            .iter()
            .filter(|file| file.path != old.path && file.new_code_lines > file.limit)
        {
            moved_code(out, &new.path, &old.path, old_text, &new.new_text);
        }
    }
}

fn moved_code(out: &mut Vec<GuardViolation>, new: &str, old: &str, old_text: &str, new_text: &str) {
    if super::similarity::ratio(old_text, new_text) > 0.75 {
        out.push(GuardViolation::new(
            new,
            format!("appears to receive displaced code from {old}"),
        ));
    }
}
