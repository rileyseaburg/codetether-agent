//! pin tool storage adapter over the shared session schema.
/// Internal context tool binding for application-local session types.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_context_tool_pin {
    ($name:ident, $bus:ty, $resolver:path) => {
        #[$crate::__private::async_trait]
        impl $crate::__private::context_pin::PinSource for $name {
            fn page_count(&self) -> usize {
                self.pages.len()
            }
            fn page_offset(&self) -> usize {
                $name::message_offset(self)
            }
            fn page_kind(
                &self,
                index: usize,
            ) -> ::std::option::Option<$crate::__private::PageKind> {
                self.pages.get(index).copied().or_else(|| {
                    self.storage
                        .0
                        .lock()
                        .unwrap()
                        .pin_sources
                        .get(index.checked_sub(self.pages.len())?)
                        .map(|source| source.page)
                })
            }
            fn source_message(
                &self,
                index: usize,
            ) -> ::std::option::Option<$crate::__private::Message> {
                self.messages.get(index).cloned().or_else(|| {
                    self.storage
                        .0
                        .lock()
                        .unwrap()
                        .pin_sources
                        .get(index.checked_sub(self.messages.len())?)
                        .map(|source| source.message.clone())
                })
            }
            fn set_page_kind(&mut self, index: usize, kind: $crate::__private::PageKind) {
                if index < self.pages.len() {
                    self.pages[index] = kind;
                } else {
                    self.storage.0.lock().unwrap().pin_sources[index - self.pages.len()].page =
                        kind;
                }
            }
            fn source_index(&self, absolute: usize) -> Option<usize> {
                if let Some(index) = absolute
                    .checked_sub($name::message_offset(self))
                    .filter(|index| *index < self.messages.len())
                {
                    return Some(index);
                }
                self.storage
                    .0
                    .lock()
                    .unwrap()
                    .pin_sources
                    .iter()
                    .position(|source| source.index == absolute)
                    .map(|index| self.messages.len() + index)
            }
            async fn prepare_sources(
                &mut self,
                indices: &[usize],
            ) -> $crate::__private::anyhow::Result<()> {
                $crate::__private::store::pin_edits::prepare(self, indices).await
            }
            fn constraint_source(
                &self,
                absolute: usize,
            ) -> Option<$crate::__private::ConstraintRecord> {
                use $crate::__private::context_pin::PinSource;
                if let Some(index) = self.source_index(absolute) {
                    if let (Some(message), Some(page)) =
                        (self.source_message(index), self.page_kind(index))
                    {
                        return $crate::__private::ConstraintRecord::from_message(
                            absolute, &message, page,
                        );
                    }
                }
                $crate::__private::store::constraints::prefix(self)
                    .into_iter()
                    .find(|source| source.source_turn == absolute)
            }
            fn corrections(&self) -> &[$crate::__private::ConstraintCorrection] {
                &self.metadata.constraint_corrections
            }
            fn set_corrections(
                &mut self,
                corrections: Vec<$crate::__private::ConstraintCorrection>,
            ) {
                self.metadata.constraint_corrections = corrections;
            }
            async fn persist(&mut self) -> $crate::__private::anyhow::Result<()> {
                $name::save(self).await
            }
        }
    };
}
