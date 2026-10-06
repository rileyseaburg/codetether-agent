import Speech
import AVFoundation

@MainActor
final class VoiceInput: ObservableObject {
    @Published var listening = false
    @Published var error: String?
    private let engine = AVAudioEngine()
    private let recognizer = SFSpeechRecognizer(locale: Locale(identifier: "en-US"))
    private var recognition: SFSpeechRecognitionTask?
    private var request: SFSpeechAudioBufferRecognitionRequest?
    private var tapped = false
    private var preparing = false
    func start(update: @escaping (String) -> Void) async {
        guard !listening && !preparing else { return }
        preparing = true; defer { preparing = false }
        guard await VoicePermissions.request() else { error = "Enable Microphone and Speech Recognition in iPhone Settings."; return }
        guard recognizer?.isAvailable == true else { error = "Speech recognition is unavailable. Try again."; return }
        do {
            error = nil
            let session = AVAudioSession.sharedInstance()
            try session.setCategory(.playAndRecord, mode: .measurement, options: [.defaultToSpeaker, .allowBluetooth])
            try session.setActive(true)
            let request = SFSpeechAudioBufferRecognitionRequest()
            request.shouldReportPartialResults = true
            self.request = request
            let input = engine.inputNode
            input.installTap(onBus: 0, bufferSize: 1024, format: input.outputFormat(forBus: 0)) { buffer, _ in request.append(buffer) }
            tapped = true
            engine.prepare()
            try engine.start()
            listening = true
            recognition = recognizer?.recognitionTask(with: request) { [weak self] result, failure in
                Task { @MainActor in
                    guard self?.listening == true else { return }
                    if let result { update(result.bestTranscription.formattedString) }
                    if result?.isFinal == true || failure != nil { self?.stop() }
                }
            }
        } catch { self.error = "Microphone could not start."; stop() }
    }
    func stop() {
        guard listening || tapped || request != nil else { return }
        engine.stop()
        if tapped { engine.inputNode.removeTap(onBus: 0); tapped = false }
        request?.endAudio(); recognition?.cancel()
        request = nil; recognition = nil; listening = false
        try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
    }
}
