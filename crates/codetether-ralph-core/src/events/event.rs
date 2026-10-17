//! RalphEvent contract.
use super::*;

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

/// Events emitted by the Ralph loop for TUI updates
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let event = codetether_ralph_core::events::RalphEvent::Error("failed".into());
/// match event { codetether_ralph_core::events::RalphEvent::Error(message) => assert_eq!(message, "failed"), _ => unreachable!() }
/// ```
pub enum RalphEvent {
    /// Ralph loop started
    Started {
        project: String,
        feature: String,
        stories: Vec<RalphStoryInfo>,
        max_iterations: usize,
    },

    /// New iteration started
    IterationStarted {
        iteration: usize,
        max_iterations: usize,
    },

    /// Story work started
    StoryStarted { story_id: String },

    /// Agent tool call for a story (basic - name only)
    StoryToolCall { story_id: String, tool_name: String },

    /// Agent tool call with full detail
    StoryToolCallDetail {
        story_id: String,
        detail: AgentToolCallDetail,
    },

    /// Agent message for a story
    StoryMessage {
        story_id: String,
        entry: AgentMessageEntry,
    },

    /// Quality check result for a story
    StoryQualityCheck {
        story_id: String,
        check_name: String,
        passed: bool,
    },

    /// Story completed
    StoryComplete { story_id: String, passed: bool },

    /// Story output text
    StoryOutput { story_id: String, output: String },

    /// Story error
    StoryError { story_id: String, error: String },

    /// Merge result for a story (parallel mode)
    StoryMerge {
        story_id: String,
        success: bool,
        summary: String,
    },

    /// Stage completed (parallel mode)
    StageComplete {
        stage: usize,
        completed: usize,
        failed: usize,
    },

    /// Ralph loop complete
    Complete {
        status: String,
        passed: usize,
        total: usize,
    },

    /// Error
    Error(String),
}
