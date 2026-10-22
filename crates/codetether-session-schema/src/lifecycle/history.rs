//! history operations on the application-local shared session schema.
/// Internal implementation of the shared session lifecycle declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_lifecycle_history {
    ($name:ident) => {
        impl $name {
            /// Append a message to the transcript and bump `updated_at`.
            pub fn add_message(&mut self, message: $crate::__private::Message) {
                let source_turn = self.message_count();
                self.metadata
                    .progress
                    .observe_message(&message, source_turn);
                $crate::__private::window::append(
                    $crate::__private::window::WorkingSet {
                        id: &self.id,
                        state: &self.storage,
                        messages: &mut self.messages,
                        pages: &mut self.pages,
                        tools: &mut self.tool_uses,
                        summaries: &mut self.summary_index,
                    },
                    message,
                );
                self.updated_at = $crate::__private::Utc::now();
            }

            /// Borrow the chat-history transcript as an immutable slice.
            ///
            /// Preferred read path now that [`Self::messages`] is scheduled for
            /// visibility tightening — see the Phase A plan for details. Callers
            /// who need mutable, append-only access should wrap the buffer in a
            /// [`History`](codetether_session_history::History) handle obtained via
            /// [`Self::history_mut`].
            ///
            /// # Examples
            ///
            /// ```rust,no_run
            /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
            /// use codetether_agent::session::Session;
            ///
            /// let session = Session::new().await.unwrap();
            /// assert!(session.history().is_empty());
            /// # });
            /// ```
            pub fn history(&self) -> &[$crate::__private::Message] {
                &self.messages
            }

            /// Borrow the per-message page sidecar.
            pub fn pages(&self) -> &[$crate::__private::PageKind] {
                &self.pages
            }

            /// Borrow the chat-history transcript as an append-only
            /// [`History`](codetether_session_history::History) handle.
            ///
            /// The returned handle can only grow the buffer via
            /// [`History::append`](codetether_session_history::History::append). Any mutation
            /// that would violate the Phase A invariant (destructive in-place
            /// rewrite) is not reachable through this API surface.
            pub fn history_mut(&mut self) -> $crate::__private::History<'_> {
                $crate::__private::History::with_pages(&mut self.messages, &mut self.pages)
            }
        }
    };
}
