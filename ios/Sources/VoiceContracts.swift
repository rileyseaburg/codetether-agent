import AVFoundation
import Foundation

/// Minimal contracts between `VoiceLoop` and its collaborators, so the
/// loop is fully drivable by test fakes without audio hardware.
@MainActor
protocol MicEngineContract {
    var isRunning: Bool { get }
    func prepare() -> Bool
    func start(onEvent: @escaping (VoiceLoopEvent) -> Void,
               onAudio: ((AVAudioPCMBuffer) -> Void)?) throws
    func stop()
    func resetDetection()
}

@MainActor
protocol TranscriberContract {
    var onUpdate: ((String) -> Void)? { get set }
    func requestPermissions() async -> Bool
    func start(onFinal: @escaping (String) -> Void)
    func stop()
    func cancel()
    func append(_ buffer: AVAudioPCMBuffer)
}

@MainActor
protocol SpeakerContract {
    func speak(_ text: String, completion: @escaping () -> Void)
    func stop()
}