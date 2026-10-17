//! Stateful threshold and reclamation decisions.

use std::time::Instant;

use super::config::Config;

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Actions {
    pub warn: bool,
    pub critical: bool,
    pub trim: bool,
}

#[derive(Debug, Default)]
pub(super) struct State {
    warned: bool,
    critical_written: bool,
    last_trim: Option<Instant>,
}

impl State {
    pub fn observe(&mut self, rss_mib: u64, now: Instant, config: Config) -> Actions {
        let warn = !self.warned && rss_mib >= config.warn_mib;
        let critical = !self.critical_written && rss_mib >= config.critical_mib;
        let trim = rss_mib >= config.warn_mib
            && self
                .last_trim
                .is_none_or(|last| now.duration_since(last) >= config.trim);
        self.warned |= warn;
        self.critical_written |= critical;
        if trim {
            self.last_trim = Some(now);
        }
        Actions {
            warn,
            critical,
            trim,
        }
    }
}
