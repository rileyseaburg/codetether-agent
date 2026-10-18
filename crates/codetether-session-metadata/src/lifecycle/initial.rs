use crate::SessionMetadata;
use chrono::{DateTime, Utc};
use codetether_history_sink::HistorySinkConfig;
use codetether_provenance::ExecutionProvenance;
use uuid::Uuid;
/// Initial identity and metadata for a newly created session.
/// # Examples
/// ```no_run
/// let initial = codetether_session_metadata::lifecycle::InitialSession::new()?;
/// assert_eq!(initial.agent, "build");
/// assert_eq!(initial.created_at, initial.updated_at);
/// # Ok::<(), std::io::Error>(())
/// ```
pub struct InitialSession {
    /// New UUID used by the transcript and its provenance.
    pub id: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Initial update timestamp, equal to creation.
    pub updated_at: DateTime<Utc>,
    /// Default owning agent persona.
    pub agent: String,
    /// Current-directory, provenance and optional history-sink configuration.
    pub metadata: SessionMetadata,
}
impl InitialSession {
    /// Allocate session identity and load initial metadata from the process.
    /// # Returns
    /// Initial fields to combine with an empty transcript and runtime state.
    /// # Errors
    /// Returns an error if the current working directory cannot be read.
    /// # Examples
    /// ```no_run
    /// let initial = codetether_session_metadata::lifecycle::InitialSession::new()?;
    /// assert!(!initial.id.is_empty());
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn new() -> std::io::Result<Self> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let provenance = Some(ExecutionProvenance::for_session(&id, "build"));
        let history_sink = HistorySinkConfig::from_env().ok().flatten();
        Ok(Self {
            id,
            created_at: now,
            updated_at: now,
            agent: "build".to_string(),
            metadata: SessionMetadata {
                directory: Some(std::env::current_dir()?),
                provenance,
                history_sink,
                ..Default::default()
            },
        })
    }
}
