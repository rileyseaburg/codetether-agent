import Foundation

/// Listening transitions and captions, separate from sending and reply playback.
extension VoiceLoop {
    func resumeListening() {
        reply = nil
        guard phase != .idle else { return }
        silence.cancel()
        transcript = ""
        mic?.resetDetection()
        phase = .listening
        transcriber?.start(onFinal: { [weak self] text in self?.finishTurn(with: text) })
    }

    func bargeIn() {
        turnID = UUID()
        speaker?.stop()
        resumeListening()
    }

    /// Update live captions only while listening to the current turn.
    func showPartial(_ text: String) {
        guard phase == .listening || phase == .userSpeaking else { return }
        guard !text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty, text != transcript else { return }
        transcript = text
        phase = .userSpeaking
        silence.wordHeard()
    }
}