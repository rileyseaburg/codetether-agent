//! construct operations on the application-local shared session schema.
/// Internal implementation of the shared session lifecycle declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_lifecycle_construct {
    ($name:ident) => {
        impl $name {
            /// Create a new empty session rooted at the current working directory.
            ///
            /// # Errors
            ///
            /// Returns an error if the current working directory cannot be resolved.
            ///
            /// # Examples
            ///
            /// ```rust,no_run
            /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
            /// use codetether_agent::session::Session;
            ///
            /// let session = Session::new().await.unwrap();
            /// assert!(!session.id.is_empty());
            /// assert_eq!(session.agent, "build");
            /// assert!(session.messages.is_empty());
            /// # });
            /// ```
            pub async fn new() -> $crate::__private::anyhow::Result<Self> {
                let initial = $crate::__private::metadata::lifecycle::InitialSession::new()?;

                Ok(Self {
                    id: initial.id,
                    title: None,
                    created_at: initial.created_at,
                    updated_at: initial.updated_at,
                    messages: Default::default(),
                    pages: Default::default(),
                    summary_index: $crate::__private::SummaryIndex::new(),
                    tool_uses: Default::default(),
                    usage: $crate::__private::Usage::default(),
                    agent: initial.agent,
                    metadata: initial.metadata,
                    max_steps: None,
                    bus: None,
                    storage: Default::default(),
                })
            }
        }
    };
}
