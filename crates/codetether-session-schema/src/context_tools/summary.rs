//! summary tool storage adapter over the shared session schema.
/// Internal context tool binding for application-local session types.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_context_tool_summary {
    ($name:ident, $bus:ty, $resolver:path) => {
        #[$crate::__private::async_trait]
        impl $crate::__private::context_summarize::SummarySource for $name {
            type Prepared = $crate::__private::context_summarize::production::Prepared;
            fn summary_index(&self) -> &$crate::__private::SummaryIndex {
                &self.summary_index
            }
            fn summary_index_mut(&mut self) -> &mut $crate::__private::SummaryIndex {
                &mut self.summary_index
            }
            fn prepare(&self) -> Self::Prepared {
                let messages = self.messages.clone();
                let generation = self.summary_index.generation();
                let rlm = self.metadata.rlm.clone();
                let session_id = self.id.clone();
                let sub_provider = self.metadata.subcall_provider.clone();
                let sub_model = self.metadata.subcall_model_name.clone();

                $crate::__private::context_summarize::production::Prepared::new(
                    messages,
                    generation,
                    rlm,
                    session_id,
                    sub_provider,
                    sub_model,
                )
            }
            async fn produce(
                prepared: Self::Prepared,
                range: $crate::__private::SummaryRange,
                target: usize,
                provider: ::std::sync::Arc<dyn $crate::__private::Provider>,
                model: &str,
            ) -> $crate::__private::anyhow::Result<$crate::__private::SummaryNode> {
                $crate::__private::context_summarize::production::produce::<$bus>(
                    prepared, range, target, provider, model, $resolver,
                )
                .await
            }
            async fn persist_summary(
                &mut self,
                range: $crate::__private::SummaryRange,
                node: &$crate::__private::SummaryNode,
            ) -> $crate::__private::anyhow::Result<()> {
                $crate::__private::store::summaries::put_snapshot(self, range, node).await
            }
        }
    };
}
