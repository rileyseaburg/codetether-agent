use super::{
    background::Background, controls::Controls, pair_task::PairTask, resources::Resources,
};
use crate::relay::Device;
use codetether_companion_desktop::Monitor;
use std::sync::{Arc, atomic::AtomicBool};

pub(super) const OFF: &str = "Capture OFF — not paired. Enter the code from your iPhone.";

pub(super) struct State {
    pub(super) controls: Option<Controls>,
    pub(super) resources: Resources,
    pub(super) monitors: Vec<Monitor>,
    pub(super) selected: Option<Monitor>,
    pub(super) status: &'static str,
    /// Last pairing failure detail; cleared on the next action.
    pub(super) pair_error: String,
    /// Memory-only device capability; dropped on Stop/unpair and Exit.
    pub(super) device: Option<Arc<Device>>,
    pub(super) background: Background,
    pub(super) pair_task: Option<PairTask>,
    pub(super) interrupt: Arc<AtomicBool>,
    pub(super) taskbar_created: u32,
    pub(super) desktop_available: bool,
}

impl State {
    pub(super) fn new(interrupt: Arc<AtomicBool>) -> Self {
        Self {
            controls: None,
            resources: Resources::default(),
            monitors: Vec::new(),
            selected: None,
            status: OFF,
            pair_error: String::new(),
            device: None,
            background: Background::default(),
            pair_task: None,
            interrupt,
            taskbar_created: 0,
            desktop_available: false,
        }
    }

    /// Text shown in the status label: the status plus any pairing failure.
    pub(super) fn display(&self) -> String {
        if self.pair_error.is_empty() {
            self.status.to_string()
        } else {
            format!("{}\n{}", self.status, self.pair_error)
        }
    }
}
