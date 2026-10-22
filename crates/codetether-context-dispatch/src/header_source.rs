//! Incremental source view whose pins already have mandatory full-text coverage.
use codetether_context_policy::{pages::PageKind, summary_index::SummaryIndex};
use codetether_incremental_derivation::{Source, session_binding::async_trait};
use codetether_provider_core::{ContentPart, Message};

pub(super) struct HeaderSource<'a, H>(pub &'a H);

#[async_trait]
impl<H: Source> Source for HeaderSource<'_, H> {
    fn messages(&self) -> &[Message] {
        self.0.messages()
    }
    fn pages(&self) -> &[PageKind] {
        self.0.pages()
    }
    fn pinned_source_in_header(&self, index: usize) -> bool {
        self.0.messages().get(index).is_some_and(|message| {
            !message.content.is_empty()
                && message.content.iter().all(
                    |part| matches!(part, ContentPart::Text { text } if !text.trim().is_empty()),
                )
        })
    }
    fn source_offset(&self) -> usize {
        self.0.source_offset()
    }
    fn summary_index(&self) -> Option<&SummaryIndex> {
        self.0.summary_index()
    }
    async fn prepared_index(&self) -> SummaryIndex {
        self.0.prepared_index().await
    }
}

#[cfg(test)]
#[path = "header_source_tests.rs"]
mod tests;
