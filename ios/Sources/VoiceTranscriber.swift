import AVFoundation
import Foundation
import Speech

/// Apple Speech transcriber restricted to on-device recognition
/// (`requiresOnDeviceRecognition = true`). Audio stays on-device; the
/// recognized text is sent to the agent only after the chosen pause.
@MainActor
final class VoiceTranscriber: ObservableObject, TranscriberContract {
    @Published var partial = ""
    @Published var error: String?
    let recognizer = SFSpeechRecognizer(locale: Locale(identifier: "en-US"))
    var task: SFSpeechRecognitionTask?
    var request: SFSpeechAudioBufferRecognitionRequest?
    var onFinal: ((String) -> Void)?
    var onUpdate: ((String) -> Void)?
    var generation = UUID()
    var accumulated = ""

    func requestPermissions() async -> Bool { await VoicePermissions.request() }

    /// The callback fires only when stop() is requested after the user's pause.
    /// Natural Apple finals roll into a new request, preserving the current turn.
    func start(onFinal: @escaping (String) -> Void) {
        cancel()
        error = nil
        self.onFinal = onFinal
        guard recognizer?.isAvailable == true, recognizer?.supportsOnDeviceRecognition == true else {
            error = "On-device speech recognition is unavailable. Check Speech and Microphone permissions."
            return
        }
        beginRecognition()
    }

    /// Appends mic audio (called from the mic tap thread; the request
    /// is thread-safe).
    func append(_ buffer: AVAudioPCMBuffer) {
        request?.append(buffer)
    }

    /// Ends audio and delivers the settled transcript to `onFinal`.
    func stop() {
        let text = partial, handler = onFinal
        cancel()
        handler?(text)
    }

    /// Teardown/restart must never deliver a stale transcript as a new turn.
    func cancel() {
        generation = UUID()
        task?.cancel(); task = nil
        request?.endAudio(); request = nil
        partial = ""; onFinal = nil
        accumulated = ""
    }
}