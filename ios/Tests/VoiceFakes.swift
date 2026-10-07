import Testing
@testable import CodeTether
import AVFoundation

@MainActor
final class FakeMic: MicEngineContract {
    var isRunning = false
    private var onEvent: ((VoiceLoopEvent) -> Void)?
    private var onAudio: ((AVAudioPCMBuffer) -> Void)?
    func prepare() -> Bool { true }
    func start(onEvent: @escaping (VoiceLoopEvent) -> Void,
               onAudio: ((AVAudioPCMBuffer) -> Void)?) throws {
        self.onEvent = onEvent; self.onAudio = onAudio; isRunning = true
    }
    func stop() { isRunning = false }
    func resetDetection() {}
    func emit(_ event: VADTurnDetector.Event, probability: Float) {
        onEvent?(.init(event: event, probability: probability))
    }
}

@MainActor
final class FakeTranscriber: TranscriberContract {
    var onFinal: ((String) -> Void)?
    var onUpdate: ((String) -> Void)?
    var text = "hello agent"
    func requestPermissions() async -> Bool { true }
    func start(onFinal: @escaping (String) -> Void) { self.onFinal = onFinal }
    func stop() { let handler = onFinal; onFinal = nil; handler?(text) }
    func cancel() { onFinal = nil }
    func hear(_ text: String) { self.text = text; onUpdate?(text) }
    func append(_ buffer: AVAudioPCMBuffer) {}
}

@MainActor
final class FakeSpeaker: SpeakerContract {
    var spoken: [String] = []
    var completions: [() -> Void] = []
    func speak(_ text: String, completion: @escaping () -> Void) {
        spoken.append(text); completions.append(completion)
    }
    func stop() { if let done = completions.popLast() { done() } }
}