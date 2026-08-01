//! Tracking of whether a Z.AI stream actually produced usable output.
//!
//! A turn that calls tools and emits no prose is a complete, successful turn:
//! the tool-call deltas *are* the output. The stream loop originally set its
//! `produced_output` flag only for `content` and `reasoning_content`, so a
//! tool-only turn looked identical to a stream that sent nothing. When such a
//! turn also ended without a `finish_reason` — the provider closing the body
//! after the last tool delta — the empty-stream fault fired and the session
//! restarted a turn the model had already finished, five times, then failed it.
//! Observed live as `frames=99, finish_reason=none` on a turn whose only work
//! was a `bash` call.
//!
//! Kinds of output are tracked separately from the fault decision so the
//! diagnostic can say *what* arrived rather than only that nothing did.

/// Records which kinds of output a single Z.AI stream has emitted.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OutputSeen {
    /// Answer text or reasoning text arrived.
    pub text: bool,
    /// At least one tool-call delta arrived.
    pub tool_calls: bool,
}

impl OutputSeen {
    /// Returns `true` when no output of any kind was observed.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::provider::zai::stream_output::OutputSeen;
    ///
    /// let mut seen = OutputSeen::default();
    /// assert!(seen.is_empty());
    ///
    /// // A tool-only turn is NOT empty: the tool call is the output.
    /// seen.tool_calls = true;
    /// assert!(!seen.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        !self.text && !self.tool_calls
    }

    /// Describes the observed output for diagnostics.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::provider::zai::stream_output::OutputSeen;
    ///
    /// assert_eq!(OutputSeen::default().describe(), "none");
    /// assert_eq!(
    ///     OutputSeen { text: false, tool_calls: true }.describe(),
    ///     "tool_calls",
    /// );
    /// ```
    pub fn describe(&self) -> &'static str {
        match (self.text, self.tool_calls) {
            (true, true) => "text+tool_calls",
            (true, false) => "text",
            (false, true) => "tool_calls",
            (false, false) => "none",
        }
    }
}

#[cfg(test)]
#[path = "zai_stream_output_tests.rs"]
mod tests;
