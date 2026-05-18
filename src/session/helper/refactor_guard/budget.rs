use super::types::{FileStatus, GuardFile, GuardReport, GuardViolation};

pub fn apply(report: &mut GuardReport) {
    let mut out = Vec::new();
    for file in &report.files {
        match file.status {
            FileStatus::Added if file.new_code_lines > file.limit => out.push(new_file(file)),
            FileStatus::Modified => modified(file, &mut out),
            FileStatus::Added => {}
        }
    }
    report.violations.extend(out);
}

fn new_file(file: &GuardFile) -> GuardViolation {
    GuardViolation::new(
        &file.path,
        format!(
            "New file has {} code lines (limit: {})",
            file.new_code_lines, file.limit
        ),
    )
}

fn modified(file: &GuardFile, out: &mut Vec<GuardViolation>) {
    let Some(old) = file.old_code_lines else {
        return;
    };
    if old <= file.limit && file.new_code_lines > file.limit {
        out.push(GuardViolation::new(
            &file.path,
            format!("File grew past {} code lines", file.limit),
        ));
    } else if old > file.limit && file.new_code_lines > old {
        out.push(GuardViolation::new(
            &file.path,
            format!("Oversized file grew ({old} -> {})", file.new_code_lines),
        ));
    }
}
