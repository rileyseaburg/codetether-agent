//! CLI syntax for MoltbookCommand.
use crate::{
    MoltbookCommentArgs, MoltbookPostArgs, MoltbookRegisterArgs, MoltbookSearchArgs,
    MoltbookUpdateProfileArgs,
};
use clap::Subcommand;
/// Parsed options for the moltbook command command.
/// # Examples
/// ```
/// use codetether_cli_options::MoltbookCommand;
/// assert!(matches!(MoltbookCommand::Status, MoltbookCommand::Status));
/// ```

#[derive(Subcommand, Debug)]
pub enum MoltbookCommand {
    /// Register a new agent on Moltbook
    Register(MoltbookRegisterArgs),

    /// Check claim status
    Status,

    /// View your Moltbook profile
    Profile,

    /// Update your profile description
    UpdateProfile(MoltbookUpdateProfileArgs),

    /// Create a post (defaults to m/general)
    Post(MoltbookPostArgs),

    /// Post a CodeTether introduction to Moltbook
    Intro,

    /// Run a heartbeat — check feed, show recent posts
    Heartbeat,

    /// Comment on a Moltbook post
    Comment(MoltbookCommentArgs),

    /// Search Moltbook posts and comments
    Search(MoltbookSearchArgs),
}
