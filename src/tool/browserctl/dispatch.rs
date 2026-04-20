use super::actions::{device, dom, dom_extra, eval, lifecycle, nav, net, tabs, upload};
use super::input::{BrowserCtlAction, BrowserCtlInput};

pub(super) async fn dispatch(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    match &input.action {
        BrowserCtlAction::Health => nav::health(input).await,
        BrowserCtlAction::Start => nav::start(input).await,
        BrowserCtlAction::Stop => nav::stop(input).await,
        BrowserCtlAction::Snapshot => nav::snapshot(input).await,
        BrowserCtlAction::Goto => nav::goto(input).await,
        BrowserCtlAction::Back => nav::back(input).await,
        BrowserCtlAction::Reload => nav::reload(input).await,
        BrowserCtlAction::Wait => lifecycle::wait(input).await,
        BrowserCtlAction::Click => dom::click(input).await,
        BrowserCtlAction::Upload => upload::upload(input).await,
        BrowserCtlAction::Fill => dom::fill(input).await,
        BrowserCtlAction::Type => dom::type_text(input).await,
        BrowserCtlAction::Press => dom::press(input).await,
        BrowserCtlAction::Text => dom::text(input).await,
        BrowserCtlAction::Html => dom::html(input).await,
        BrowserCtlAction::ClickText => dom_extra::click_text(input).await,
        BrowserCtlAction::FillNative => dom_extra::fill_native(input).await,
        BrowserCtlAction::Toggle => dom_extra::toggle(input).await,
        BrowserCtlAction::Eval => eval::eval(input).await,
        BrowserCtlAction::MouseClick => device::mouse_click(input).await,
        BrowserCtlAction::KeyboardType => device::keyboard_type(input).await,
        BrowserCtlAction::KeyboardPress => device::keyboard_press(input).await,
        BrowserCtlAction::Screenshot => lifecycle::screenshot(input).await,
        BrowserCtlAction::Tabs => tabs::tabs(input).await,
        BrowserCtlAction::TabsSelect => tabs::tabs_select(input).await,
        BrowserCtlAction::TabsNew => tabs::tabs_new(input).await,
        BrowserCtlAction::TabsClose => tabs::tabs_close(input).await,
        BrowserCtlAction::NetworkLog => net::network_log(input).await,
        BrowserCtlAction::Fetch => net::fetch(input).await,
        BrowserCtlAction::Axios => net::axios(input).await,
        BrowserCtlAction::Xhr => net::xhr(input).await,
        BrowserCtlAction::Diagnose => net::diagnose(input).await,
        BrowserCtlAction::Detect => unreachable!("Detect is handled before dispatch"),
    }
}
