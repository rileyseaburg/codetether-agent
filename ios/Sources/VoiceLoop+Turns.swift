import Foundation

/// Turn handling for `VoiceLoop` (split for file-size limits).
extension VoiceLoop {
    /// Forces end-of-turn: freeze the transcript and hand it to the agent.
    func endTurnNow() {
        guard phase == .listening || phase == .userSpeaking else { return }
        silence.cancel()
        pendingFinal = true
        turnID = UUID(); let current = turnID
        phase = .thinking
        transcriber?.stop()   // delivers settled transcript via onFinal
        Task { @MainActor in
            try? await Task.sleep(nanoseconds: 800_000_000)
            guard self.pendingFinal, self.turnID == current else { return }
            self.pendingFinal = false
            self.resumeListening()   // empty transcript: never sent, loop again
        }
    }

    /// Delivers the settled transcript (from the transcriber's onFinal).
    /// A natural `isFinal` during listening counts as end-of-turn; stray
    /// finals after `loop.stop()` (phase idle) are ignored.
    func finishTurn(with text: String) {
        let live = phase == .listening || phase == .userSpeaking
        guard pendingFinal || live else { return }
        pendingFinal = false
        let settled = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !settled.isEmpty else { resumeListening(); return }
        phase = .thinking
        silence.cancel(); transcriber?.cancel()
        transcript = settled
        let sendFn = send
        turnID = UUID(); let current = turnID
        micTask?.cancel()
        micTask = Task {
            guard !Task.isCancelled else { return }
            let answer = await sendFn?(settled) ?? nil
            guard !Task.isCancelled, current == turnID, phase == .thinking else { return }
            if let answer {
                reply = answer; phase = .speaking
                speakStartedAt = Date()
                speaker?.speak(answer) { [weak self] in
                    guard let self, self.turnID == current, self.phase == .speaking else { return }
                    self.resumeListening()
                }
            } else { resumeListening() }
        }
    }

}