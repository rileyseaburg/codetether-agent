//! Session fields and storage required for non-destructive prefix branching.
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;
use codetether_session_metadata::SessionMetadata;
/// Application-owned session binding; save persists only this instance.
/// # Implementors
/// Native sessions expose fields while this crate owns branch policy.
#[async_trait::async_trait]
pub trait ForkSession: Sized + Send + Sync {
    /// Create a fresh durable identity.
    /// # Errors
    /// Returns native initialization/storage errors.
    async fn new() -> anyhow::Result<Self>;
    /// Load an existing conversation without mutating it.
    /// # Errors
    /// Returns missing or corrupt source errors.
    async fn load(id: &str) -> anyhow::Result<Self>;
    /// Save this branch without writing the source conversation.
    /// # Errors
    /// Returns durable persistence errors.
    async fn save(&self) -> anyhow::Result<()>;
    /// Return source messages in their stored order.
    fn messages(&self) -> &[Message];
    /// Absolute position of the first loaded message in the parent transcript.
    fn message_offset(&self) -> usize;
    /// Page classifications aligned with the loaded parent messages.
    fn page_kinds(&self) -> &[PageKind];
    /// Install page classifications aligned with the new child's messages.
    fn replace_pages(&mut self, pages: Vec<PageKind>);
    /// Return the optional conversation title.
    fn title(&self) -> Option<&str>;
    /// Return the current metadata.
    fn metadata(&self) -> &SessionMetadata;
    /// Return the durable identity.
    fn id(&self) -> &str;
    /// Return the configured agent name.
    fn agent(&self) -> &str;
    /// Set the new branch title.
    fn set_title(&mut self, title: Option<String>);
    /// Install sanitized inherited metadata.
    fn set_metadata(&mut self, metadata: SessionMetadata);
    /// Set the branch agent using normal native bookkeeping.
    fn set_agent_name(&mut self, agent: &str);
    /// Append a source-context message using normal native bookkeeping.
    fn add_message(&mut self, message: Message);
}
