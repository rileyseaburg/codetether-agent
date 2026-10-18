//! Bind the shared runtime to native thinker services and executable tools.
pub type Native<R, const EMPTY_STATE: bool> =
    codetether_cognition_runtime::application::ApplicationHost<
        super::thinker::Services,
        R,
        EMPTY_STATE,
    >;
