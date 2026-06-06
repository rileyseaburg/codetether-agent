use super::WorktreeManager;

const CORRUPTION_MARKERS: &[&str] = &[
    "missing blob",
    "missing tree",
    "missing commit",
    "bad object",
    "unable to read",
    "object file",
    "hash mismatch",
    "broken link from",
    "corrupt",
    "invalid sha1 pointer",
    "fatal: loose object",
    "failed to parse commit",
];

impl WorktreeManager {
    pub(crate) fn combined_output(stdout: &[u8], stderr: &[u8]) -> String {
        let left = String::from_utf8_lossy(stdout);
        let right = String::from_utf8_lossy(stderr);
        format!("{left}\n{right}")
    }

    pub(crate) fn looks_like_object_corruption(output: &str) -> bool {
        let lower = output.to_ascii_lowercase();
        CORRUPTION_MARKERS
            .iter()
            .any(|marker| lower.contains(marker))
    }

    pub(crate) fn summarize_git_output(output: &str) -> String {
        output
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .map(|line| line.chars().take(220).collect::<String>())
            .unwrap_or_else(|| "git command reported no details".to_string())
    }
}
