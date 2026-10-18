//! Project an application-local session using the existing workspace canonicalizer.
/// Declare a projection function over the existing session fields.
/// # Usage
/// ```text
/// codetether_recall_index::define_session_projection!(pub fn project_session(Session), canonical);
/// ```
#[macro_export]
macro_rules! define_session_projection {
    ($visibility:vis fn $name:ident($session:ty), $canonical:path) => {
$visibility fn $name(session: &$session) -> ::std::option::Option<$crate::IndexedSession> {
    let workspace = $canonical(session.metadata.directory.as_deref()?);
    Some($crate::project(
        $crate::ProjectionInput {
            messages: &session.messages,
            session_id: &session.id,
            title: session.title.as_deref(),
            workspace,
            updated_at: session.updated_at,
            message_offset: session.message_offset(),
            message_count: session.message_count(),
        },
    ))
}

    };
}
