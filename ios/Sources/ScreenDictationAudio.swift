import AVFoundation

/// Microphone ownership for Screen dictation only; no recording files or network audio.
@MainActor
final class ScreenDictationAudio {
    private let engine = AVAudioEngine()
    private var tapped = false
    private var activated = false

    func start(onAudio: @escaping (AVAudioPCMBuffer) -> Void) throws {
        stop()
        let session = AVAudioSession.sharedInstance()
        try session.setCategory(.playAndRecord, mode: .measurement,
                                options: [.defaultToSpeaker, .allowBluetooth])
        try session.setActive(true)
        activated = true
        do {
            let format = engine.inputNode.outputFormat(forBus: 0)
            guard format.sampleRate > 0, format.channelCount > 0 else { throw AudioError.noInput }
            engine.inputNode.installTap(onBus: 0, bufferSize: 1024, format: format) { buffer, _ in
                if let owned = VoiceAudioBuffer.copy(buffer) { onAudio(owned) }
            }
            tapped = true
            engine.prepare()
            try engine.start()
        } catch {
            stop()
            throw error
        }
    }

    func stop() {
        if tapped { engine.inputNode.removeTap(onBus: 0); tapped = false }
        engine.stop()
        if activated {
            try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
            activated = false
        }
    }

    private enum AudioError: Error { case noInput }
}
