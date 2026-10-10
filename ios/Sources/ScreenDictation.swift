import Foundation
import Combine

/// On-device dictation into the Screen question draft; never auto-sends.
@MainActor
final class ScreenDictation: ObservableObject {
    @Published private(set) var listening = false
    @Published var error: String?
    private let transcriber = VoiceTranscriber()
    private let audio = ScreenDictationAudio()

    func toggle(update: @escaping (String) -> Void) async {
        if listening { stop(); return }
        guard await transcriber.requestPermissions() else {
            error = "Enable Microphone and Speech Recognition in iPhone Settings."; return
        }
        error = nil
        transcriber.onUpdate = { text in update(text) }
        transcriber.start { text in if !text.isEmpty { update(text) } }
        if let failure = transcriber.error { error = failure; return }
        do {
            let sink = transcriber
            try audio.start { buffer in Task { @MainActor in sink.append(buffer) } }
            listening = true
        } catch {
            transcriber.cancel()
            self.error = "Microphone could not start."
        }
    }

    func stop() {
        guard listening else { return }
        audio.stop()
        transcriber.stop()
        transcriber.onUpdate = nil
        listening = false
    }
}
