import Foundation

/// VAD ingest for `VoiceMicEngine` (split for file-size limits): turns
/// resampled 16 kHz mono audio into energy-gated Silero probabilities and
/// hysteresis turn events.
extension VoiceMicEngine {
    func ingest(_ samples: [Float]) {
        guard isRunning else { return }
        for frame in frames.append(mono: samples) {
            let level = VADEnergyGate.levelDb(frame)
            inputLevelDb = level
            _ = gate.push(level)
            guard let vad else { continue }
            let prob: Float
            do { prob = try vad.process(frame) } catch { prob = 0 }
            speechProbability = gate.isOpen ? purify(prob) : 0
            let events = detector.feed(speechProbability)
            // Quiet frames must reach the detector or speechEnd never fires.
            if events.isEmpty { onEvent?(.init(probability: speechProbability)) }
            for event in events { onEvent?(.init(event: event, probability: speechProbability)) }
        }
    }
}

/// Coalesces NaN/inf VAD output to 0 (defensive; CoreML float output).
func purify(_ value: Float) -> Float {
    value.isFinite ? max(0, min(1, value)) : 0
}