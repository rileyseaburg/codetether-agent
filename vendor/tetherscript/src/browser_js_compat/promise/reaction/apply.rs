use super::{super::*, types::Reaction};

pub(super) fn settle(item: Reaction, current: state::PromiseState) {
    match item.kind {
        super::types::Kind::Then { ok, err } => {
            then::settle_reaction(ok, err, current, item.state, item.object, item.queue)
        }
        super::types::Kind::Finally { callback } => {
            finally::settle_reaction(callback, current, item.state, item.object, item.queue)
        }
    }
}
