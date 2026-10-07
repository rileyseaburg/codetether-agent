import Foundation

/// VAD-event routing for the voice loop: translates detector events into
/// phase transitions and turn actions, phase-gated so stale events from a
/// previous phase can't fire.
extension VoiceLoop {
    /// True only in idle or a failed state (either may restart the loop).
    var canStart: Bool {
        if case .idle = phase { return true }
        if case .failed = phase { return true }
        return false
    }

    func handle(_ event: VoiceLoopEvent) {
        probability = event.probability
        guard let e = event.event else { return }
        switch phase {
        case .listening where e == .speechStart: phase = .userSpeaking
        // Only the last-word deadline ends a turn, never a short VAD silence.
        // Do not recognize speaker output as a new turn; resume after Kokoro finishes.
        case .listening, .userSpeaking, .thinking, .speaking, .idle, .failed:
            break
        }
    }

    func bargeAllowed(_ p: Float) -> Bool {
        Date().timeIntervalSince(speakStartedAt) > 0.8 && p > 0.8
    }
}