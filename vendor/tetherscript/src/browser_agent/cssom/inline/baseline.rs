//! Baseline alignment for inline fragments within a line box.

use super::line::LineBox;

/// Align all fragments in the line to a shared baseline.
///
/// The baseline is the maximum ascent (baseline_offset) of all fragments.
/// Each fragment's y is adjusted so its baseline sits on the shared baseline.
pub fn align_baselines(line: &mut LineBox) {
    if line.fragments.is_empty() {
        line.height = 0;
        line.baseline = 0;
        return;
    }

    let baseline = line
        .fragments
        .iter()
        .map(|f| f.baseline_offset)
        .max()
        .unwrap_or(0);
    let descent = line
        .fragments
        .iter()
        .map(|f| f.height - f.baseline_offset)
        .max()
        .unwrap_or(0);

    line.baseline = baseline;
    line.height = baseline + descent;
    for fragment in &mut line.fragments {
        fragment.y = line.y + baseline - fragment.baseline_offset;
    }
}
