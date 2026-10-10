use super::{font::Font, session::SessionWatch, timer::Timer, tray::Tray};

#[derive(Default)]
pub(super) struct Resources {
    pub(super) font: Option<Font>,
    pub(super) session: Option<SessionWatch>,
    pub(super) timer: Option<Timer>,
    pub(super) tray: Option<Tray>,
}

impl Resources {
    pub(super) fn stop_observers(&mut self) {
        self.timer = None;
        self.session = None;
        self.tray = None;
    }
}
