import Foundation

/// Background playback is allowed; background microphone capture is not.
extension VoiceLoop {
    func enterBackground() {
        isBackgrounded = true
        silence.cancel()
        pendingFinal = false
        transcriber?.cancel()
        guard phase == .speaking else { stop(); return }
        mic?.suspendCapture()
        probability = 0
    }

    func enterForeground() {
        isBackgrounded = false
        // An interrupted listening session requires an explicit tap to resume.
    }
}