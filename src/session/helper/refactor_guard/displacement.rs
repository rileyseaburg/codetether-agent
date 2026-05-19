use super::types::{GuardReport, GuardViolation};

pub fn apply(report: &mut GuardReport) {
    let mut out = Vec::new();
    wrappers(report, &mut out);
    super::displacement_similar::apply(report, &mut out);
    report.violations.extend(out);
}

fn wrappers(report: &GuardReport, out: &mut Vec<GuardViolation>) {
    for wrapper in report
        .files
        .iter()
        .filter(|file| file.wrapper_target.is_some())
    {
        let target = wrapper.wrapper_target.as_ref().unwrap();
        let moved = report
            .files
            .iter()
            .find(|file| super::wrapper::stem(&file.path).as_deref() == Some(target.as_str()));
        if let Some(moved) = moved.filter(|file| file.new_code_lines > file.limit) {
            out.push(GuardViolation::new(
                &wrapper.path,
                format!(
                    "became a wrapper while {} has {} lines",
                    moved.path, moved.new_code_lines
                ),
            ));
        }
    }
}
