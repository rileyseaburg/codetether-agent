//! LLM-facing contract for session recall.

pub(super) const DESCRIPTION: &str = "\
OPTIONAL LOCAL RECALL FROM YOUR OWN PAST SESSIONS. Follow the user's current source and access \
instructions first. Never use this tool after the user prohibits session/history access; that \
restriction persists until explicitly revoked. Use only when a necessary detail is absent from \
the active conversation and user-designated repository sources. Pass `query`; optional \
`session_id`, `limit` (default 3, max 5), or `mode`. Evidence mode is local and immediate; \
answer mode explicitly requests slower RLM synthesis. Distinct from curated `memory`.";
