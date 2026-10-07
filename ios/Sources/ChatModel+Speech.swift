import Foundation

/// Route automatic reply audio to one player, without changing the Read aloud preference.
extension ChatModel {
    var shouldReadChatReply: Bool { !voiceModeActive && replySpeechOwner == .chat }

    func prepareReplySpeech(for owner: ReplySpeechOwner) {
        replySpeechOwner = owner
        replyForSpeech = nil
    }

    /// VoiceLoop already speaks its awaited answer; never publish it to Chat's listener.
    func publishReplySpeech(_ text: String) {
        if replySpeechOwner == .chat {
            replyForSpeech = text
        }
    }
}