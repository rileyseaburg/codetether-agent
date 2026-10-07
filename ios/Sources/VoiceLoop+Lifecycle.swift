import Foundation

/// Lifecycle extras for `VoiceLoop`: teardown ordering and the
/// context-sensitive orb-tap action.
extension VoiceLoop {
    /// Exits the loop: idle *before* collaborators stop so the settled-
    /// transcript callback finds nothing to send.
    func stop() {
        turnID = UUID()
        silence.cancel()
        micTask?.cancel(); micTask = nil
        phase = .idle; transcript = ""; reply = nil
        pendingFinal = false; probability = 0
        mic?.stop(); mic?.resetDetection()
        speaker?.stop()
        transcriber?.cancel()
    }

    /// User tapped the orb: context-sensitive action.
    func tapped() {
        switch phase {
        case .listening, .userSpeaking: endTurnNow()
        case .thinking:
            turnID = UUID(); micTask?.cancel(); stopAgent?(); resumeListening()
        case .speaking: bargeIn()
        case .idle, .failed: Task { await start() }
        }
    }
}