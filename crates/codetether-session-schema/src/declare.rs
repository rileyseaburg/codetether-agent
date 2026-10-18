//! Generate the native session type from one shared field and serialization schema.

/// Declare a session with the shared durable fields and runtime-only state.
///
/// The resulting type stays local to the invoking crate so its existing inherent
/// methods remain valid. Field order, bounded deserialization and skipped runtime
/// fields are part of this schema. Depend on this crate under its canonical name
/// `codetether_session_schema`, which the serde attributes use for resolution.
///
/// # Examples
/// ```rust
/// codetether_session_schema::declare_session!(pub struct Session;);
/// let session = Session::default();
/// assert!(session.messages.is_empty());
/// ```
#[macro_export]
macro_rules! declare_session {
    ($visibility:vis struct $name:ident;) => {
        /// A conversation session.
        ///
        /// The declaring application supplies the lifecycle and persistence methods.
        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::default::Default, $crate::__private::Serialize, $crate::__private::Deserialize)]
        #[serde(crate = "codetether_session_schema::__private::serde")]
        $visibility struct $name {
            /// UUID identifying this session on disk.
            pub id: ::std::string::String,
            /// Optional human-readable title. Auto-generated from the first user
            /// message when absent by the application’s title generation.
            pub title: ::std::option::Option<::std::string::String>,
            /// When the session was first created.
            pub created_at: $crate::__private::DateTime<$crate::__private::Utc>,
            /// When the session was last modified.
            pub updated_at: $crate::__private::DateTime<$crate::__private::Utc>,
            /// Durable session configuration.
            ///
            /// Serialized *before* [`Self::messages`] so metadata readers can identify
            /// the workspace before reaching the transcript.
            pub metadata: $crate::__private::SessionMetadata,
            /// Name of the agent persona that owns this session.
            pub agent: ::std::string::String,
            /// Ordered conversation transcript.
            #[serde(deserialize_with = "codetether_session_schema::__private::tail")]
            pub messages: $crate::__private::TrackedVec<$crate::__private::Message>,
            /// Per-message page classification sidecar.
            ///
            /// Backfilled on load for legacy sessions that predate the
            /// history/context split.
            #[serde(default)]
            pub pages: $crate::__private::TrackedVec<$crate::__private::PageKind>,
            /// Hierarchical summary cache used as the seed for proactive RLM
            /// preparation and invalidated when the application appends messages.
            #[serde(default)]
            pub summary_index: $crate::__private::SummaryIndex,
            /// Per-tool-call audit records.
            #[serde(deserialize_with = "codetether_session_schema::__private::tail")]
            pub tool_uses: $crate::__private::TrackedVec<$crate::__private::ToolUse>,
            /// Aggregate token usage across all completions in this session.
            pub usage: $crate::__private::Usage,
            /// Maximum agentic loop steps. [`None`] allows the turn to continue until
            /// completion or an explicit cancellation.
            #[serde(skip)]
            pub max_steps: ::std::option::Option<usize>,
            /// Optional bus for publishing agent thinking/reasoning.
            #[serde(skip)]
            pub bus: ::std::option::Option<::std::sync::Arc<$crate::__private::AgentBus>>,
            /// Optimistic revision and loaded-window offsets; omitted from JSON exports.
            #[serde(skip)]
            pub storage: $crate::__private::State,
        }

    };
}
