//! Top-level action dispatcher: maps [`BrowserCtlAction`] to the handler
//! in the matching `actions::*` submodule.

use super::actions::{Ctx, Outcome, device, dom, eval, lifecycle, nav, tabs};
use super::input::BrowserCtlAction;
use anyhow::Result;

pub(super) async fn dispatch(ctx: &Ctx<'_>) -> Result<Outcome> {
    use BrowserCtlAction as A;
    match ctx.input.action {
        A::Health => nav::health(ctx).await,
        A::Start => nav::start(ctx).await,
        A::Stop => nav::stop(ctx).await,
        A::Snapshot => nav::snapshot(ctx).await,
        A::Console => nav::console(ctx).await,
        A::Goto => nav::goto(ctx).await,
        A::Back => nav::back(ctx).await,
        A::Reload => nav::reload(ctx).await,
        A::Click => dom::click(ctx).await,
        A::Fill => dom::fill(ctx).await,
        A::Type => dom::type_text(ctx).await,
        A::Press => dom::press(ctx).await,
        A::Text => dom::text(ctx).await,
        A::Html => dom::html(ctx).await,
        A::ClickText => dom::click_text(ctx).await,
        A::FillNative => dom::fill_native(ctx).await,
        A::Toggle => dom::toggle(ctx).await,
        A::Eval => eval::eval(ctx).await,
        A::ConsoleEval => eval::console_eval(ctx).await,
        A::MouseClick => device::mouse_click(ctx).await,
        A::KeyboardType => device::keyboard_type(ctx).await,
        A::KeyboardPress => device::keyboard_press(ctx).await,
        A::Wait => lifecycle::wait(ctx).await,
        A::Screenshot => lifecycle::screenshot(ctx).await,
        A::Tabs => tabs::tabs(ctx).await,
        A::TabsSelect => tabs::tabs_select(ctx).await,
        A::TabsNew => tabs::tabs_new(ctx).await,
        A::TabsClose => tabs::tabs_close(ctx).await,
    }
}
