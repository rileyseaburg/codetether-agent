use super::message_queue::FrameMessageQueue;
use super::{FrameId, FrameMessage, WindowOpener};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct FrameWindowState {
    opener: Option<WindowOpener>,
    messages: FrameMessageQueue,
}

impl FrameWindowState {
    pub(crate) fn opener(&self) -> Option<WindowOpener> {
        self.opener
    }

    pub(crate) fn set_opener(&mut self, opener: WindowOpener) {
        self.opener = Some(opener);
    }

    pub(crate) fn clear_opener(&mut self) {
        self.opener = None;
    }

    pub(crate) fn push_message(&mut self, message: FrameMessage) -> FrameMessage {
        self.messages.push(message)
    }

    pub(crate) fn messages(&self) -> &[FrameMessage] {
        self.messages.messages()
    }

    pub(crate) fn messages_for(&self, target: FrameId) -> Vec<FrameMessage> {
        self.messages.for_target(target)
    }

    pub(crate) fn take_messages_for(&mut self, target: FrameId) -> Vec<FrameMessage> {
        self.messages.take_for_target(target)
    }

    pub(crate) fn clear_messages(&mut self) {
        self.messages.clear();
    }
}
