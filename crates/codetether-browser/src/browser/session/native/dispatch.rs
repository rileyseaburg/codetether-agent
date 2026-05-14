use crate::browser::{BrowserCommand as Command, BrowserError, BrowserOutput};

pub(super) async fn run(
    session: &super::super::BrowserSession,
    command: Command,
) -> Result<BrowserOutput, BrowserError> {
    match command {
        Command::Health => super::lifecycle::health(session).await,
        Command::Start(request) => super::lifecycle::start(session, request).await,
        Command::Stop => super::lifecycle::stop(session).await,
        Command::Snapshot => super::content::snapshot(session).await,
        Command::Goto(request) => super::navigation::goto(session, request).await,
        Command::Back => super::navigation::back(session).await,
        Command::Reload => super::navigation::reload(session).await,
        Command::Wait(request) => super::wait::run(session, request).await,
        Command::Click(request) => super::dom::click(session, request).await,
        Command::Hover(request) => super::dom::hover(session, request).await,
        Command::Focus(request) => super::dom::focus(session, request).await,
        Command::Blur(request) => super::dom::blur(session, request).await,
        Command::Scroll(request) => super::dom::scroll(session, request).await,
        Command::Upload(request) => super::dom::upload(session, request).await,
        Command::Fill(request) | Command::FillNative(request) => {
            super::dom::fill(session, request).await
        }
        Command::Type(request) => super::dom::type_text(session, request).await,
        Command::Press(request) => super::dom::press(session, request).await,
        Command::Text(request) => super::content::text(session, request).await,
        Command::Html(request) => super::content::html(session, request).await,
        Command::Eval(request) => super::eval::run(session, request).await,
        Command::ClickText(request) => super::dom::click_text(session, request).await,
        Command::Toggle(request) => super::dom::toggle(session, request).await,
        Command::Screenshot(request) => super::screen::capture(session, request).await,
        Command::Tabs => super::tabs::list(session).await,
        Command::TabsSelect(request) => super::tabs::select(session, request).await,
        Command::TabsNew(request) => super::tabs::new(session, request).await,
        Command::TabsClose(request) => super::tabs::close(session, request).await,
        Command::NetworkLog(request) => super::net::log(session, request).await,
        Command::Fetch(request) => super::net::fetch(session, request).await,
        Command::Axios(request) => super::net::axios(session, request).await,
        Command::Xhr(request) => super::net::xhr(session, request).await,
        Command::Replay(request) => super::net::replay(session, request).await,
        Command::Diagnose(request) => super::net::diagnose(session, request).await,
        Command::MouseClick(request) => super::device::mouse_click(session, request).await,
        Command::KeyboardType(request) => super::device::keyboard_type(session, request).await,
        Command::KeyboardPress(request) => super::device::keyboard_press(session, request).await,
    }
}
