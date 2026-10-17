//! Request framing for explicit session-recall answer synthesis.

pub(super) fn frame(query: &str, context: &str) -> String {
    format!(
        "Recall task: {query}\n\nUse the session transcript below \
         to answer the recall task. Quote short passages verbatim \
         when useful; otherwise summarise.\n\n{context}"
    )
}
