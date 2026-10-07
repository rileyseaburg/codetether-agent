import Speech

/// Restart Apple's finite recognition requests without stopping the user's conversation.
extension VoiceTranscriber {
    func beginRecognition() {
        generation = UUID()
        let current = generation
        let audio = SFSpeechAudioBufferRecognitionRequest()
        audio.shouldReportPartialResults = true
        audio.requiresOnDeviceRecognition = true
        request = audio
        task = recognizer?.recognitionTask(with: audio) { [weak self] result, failure in
            let text = result?.bestTranscription.formattedString
            let final = result?.isFinal == true
            Task { @MainActor in
                guard let self, self.generation == current else { return }
                self.recognized(text, final: final, failed: failure != nil)
            }
        }
    }

    func recognized(_ text: String?, final: Bool, failed: Bool) {
        if let text, !text.isEmpty {
            partial = accumulated.isEmpty ? text : accumulated + " " + text
            onUpdate?(partial)
        }
        if final {
            accumulated = partial
            generation = UUID()
            request?.endAudio(); task?.cancel()
            beginRecognition()
        } else if failed {
            cancel()
            error = "Speech recognition stopped. Tap to reconnect the microphone."
        }
    }
}