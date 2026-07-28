use super::{FrameId, FrameMessage};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct FrameMessageQueue {
    messages: Vec<FrameMessage>,
    next_sequence: u64,
}

impl FrameMessageQueue {
    pub(crate) fn push(&mut self, mut message: FrameMessage) -> FrameMessage {
        message.sequence = self.next_sequence;
        self.next_sequence += 1;
        self.messages.push(message.clone());
        message
    }

    pub(crate) fn messages(&self) -> &[FrameMessage] {
        &self.messages
    }

    pub(crate) fn for_target(&self, target: FrameId) -> Vec<FrameMessage> {
        self.messages
            .iter()
            .filter(|message| message.target == target)
            .cloned()
            .collect()
    }

    pub(crate) fn take_for_target(&mut self, target: FrameId) -> Vec<FrameMessage> {
        let mut taken = Vec::new();
        self.messages.retain(|message| {
            if message.target == target {
                taken.push(message.clone());
                false
            } else {
                true
            }
        });
        taken
    }

    pub(crate) fn clear(&mut self) {
        self.messages.clear();
    }
}
