import AVFoundation
import Foundation

/// Owns the audio session and a single native-rate input tap that feeds
/// both Silero VAD (resampled 16 kHz mono frames) and Apple Speech
/// (native buffer via `onAudio`). Explicit speaker routing avoids the receiver;
/// the loop suspends transcription during playback. VAD ingest is separate.
@MainActor
final class VoiceMicEngine: ObservableObject, MicEngineContract {
    // Internal setters: VoiceMicEngine+VAD (separate file) mutates these.
    @Published var speechProbability: Float = 0
    @Published var inputLevelDb: Float = -120
    let engine = AVAudioEngine()
    let routing = VoiceAudioRouting()
    var vad: SileroVAD?
    var detector: VADTurnDetector
    var gate = VADEnergyGate()
    var frames = VADFrameBuffer()
    var resampler: AudioResampler?
    var onEvent: ((VoiceLoopEvent) -> Void)?
    var onAudio: ((AVAudioPCMBuffer) -> Void)?

    init(detector: VADTurnDetector = VADTurnDetector()) {
        self.detector = detector
    }

    var isRunning: Bool { engine.isRunning }

    /// Loads the bundled Silero model. Returns false when unavailable
    /// auto-send still works using recognized-word timing when VAD is unavailable.
    func prepare() -> Bool {
        vad = try? SileroVAD()
        return vad != nil
    }
}