//! Append the last thirty lines of a nonempty progress file when available.

use crate::ActiveGoRun;
pub(crate) fn append(output: &mut String, run: &ActiveGoRun) {
    if let Ok(progress) = std::fs::read_to_string(&run.progress_filename)
        && !progress.trim().is_empty()
    {
        // Show last 30 lines to avoid overwhelming output
        let lines: Vec<&str> = progress.lines().collect();
        let start = lines.len().saturating_sub(30);
        let tail: String = lines[start..].join("\n");
        output.push_str(&format!(
            "\n## Progress Notes (last {} lines)\n\n```\n{tail}\n```\n",
            lines.len().min(30)
        ));
    }
}
