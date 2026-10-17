//! Loop check abort.

use crate::router::types::CrateAutoProcessContext;

pub(in crate::router::auto_loop) fn check_abort(ctx: &CrateAutoProcessContext<'_>) -> bool {
    ctx.abort.as_ref().is_some_and(|r| *r.borrow())
}
