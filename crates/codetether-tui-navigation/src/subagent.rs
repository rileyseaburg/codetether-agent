//! subagent navigation policy.
use crate::{Action, Host};

pub(super) fn escape(app: &mut impl Host) {
    if app.context().subagent_detail {
        app.apply(Action::SubagentDetail(false));
        app.apply(Action::SubagentScroll(0));
        app.apply(Action::Status("Agent dashboard".to_string()));
    } else {
        app.apply(Action::ReturnToChat);
    }
}

pub(super) fn up(app: &mut impl Host) {
    if app.context().subagent_detail {
        app.apply(Action::SubagentScroll(
            app.context().subagent_scroll.saturating_sub(1),
        ));
    } else {
        app.apply(Action::ChildPrevious);
    }
}

pub(super) fn down(app: &mut impl Host) {
    if app.context().subagent_detail {
        app.apply(Action::SubagentScroll(
            app.context().subagent_scroll.saturating_add(1),
        ));
    } else {
        app.apply(Action::ChildNext);
    }
}

pub(super) fn page_up(app: &mut impl Host) {
    app.apply(Action::SubagentScroll(
        app.context().subagent_scroll.saturating_sub(10),
    ));
}

pub(super) fn page_down(app: &mut impl Host) {
    app.apply(Action::SubagentScroll(
        app.context().subagent_scroll.saturating_add(10),
    ));
}
