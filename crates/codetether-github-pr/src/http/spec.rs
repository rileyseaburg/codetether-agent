//! Caller-provided PR content and creation settings.
/// Values used to create a PR or update an existing PR's title and body.
///
/// # Examples
/// ```
/// let spec = codetether_github_pr::http::PullRequestSpec {
///     title: "change", base: "main", body: "description", draft: true,
/// };
/// assert!(spec.draft);
/// ```
pub struct PullRequestSpec<'a> {
    /// Desired title.
    pub title: &'a str,
    /// Base branch for creation.
    pub base: &'a str,
    /// Fully assembled Markdown body.
    pub body: &'a str,
    /// Whether a newly created PR starts as a draft.
    pub draft: bool,
}
