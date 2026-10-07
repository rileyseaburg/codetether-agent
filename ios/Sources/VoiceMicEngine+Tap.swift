import AVFoundation

/// Tap lifecycle for `VoiceMicEngine` (split for file-size limits):
/// session setup, tap install/removal, and detection-state reset.
extension VoiceMicEngine {
    /// Starts the tap; throws on session or engine failure.
    func start(onEvent: @escaping (VoiceLoopEvent) -> Void,
               onAudio: ((AVAudioPCMBuffer) -> Void)? = nil) throws {
        guard !engine.isRunning else { return }
        self.onEvent = onEvent
        self.onAudio = onAudio
        vad?.reset(); detector.reset(); frames.reset()
        try routing.activate()
        let session = AVAudioSession.sharedInstance()
        try session.setPreferredIOBufferDuration(0.128)
        let format = engine.inputNode.outputFormat(forBus: 0)
        resampler = AudioResampler(from: format)
        // Capture locals: the tap closure runs on an internal audio thread,
        // so it must not touch @MainActor-isolated state synchronously.
        let audioTap = onAudio
        let convert = resampler
        engine.inputNode.installTap(onBus: 0, bufferSize: 2048, format: format) { [weak self] buffer, _ in
            if let owned = VoiceAudioBuffer.copy(buffer) { audioTap?(owned) }
            guard let mono = convert?.convert(buffer) else { return }
            Task { @MainActor in self?.ingest(mono) }
        }
        engine.prepare()
        do { try engine.start() } catch {
            engine.inputNode.removeTap(onBus: 0)
            throw error
        }
    }

    func stop() {
        routing.stop()
        guard engine.isRunning else { return }
        engine.inputNode.removeTap(onBus: 0)
        engine.stop()
        onAudio = nil
        try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
    }

    /// Clears turn-detection state (after barge-in or a new turn).
    func resetDetection() {
        detector.reset(); frames.reset(); gate = VADEnergyGate()
    }
}