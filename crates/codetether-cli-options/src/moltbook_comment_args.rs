//! CLI syntax for MoltbookCommentArgs.
use clap::Parser;
/// Parsed options for the moltbook comment args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::MoltbookCommentArgs::try_parse_from(["comment","id","Text"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct MoltbookCommentArgs {
    /// Post ID to comment on
    pub post_id: String,

    /// Comment content
    pub content: String,
}
