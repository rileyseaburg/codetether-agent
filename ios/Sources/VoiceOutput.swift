import Foundation

@MainActor
final class VoiceOutput: ObservableObject {
    @Published var enabled = UserDefaults.standard.bool(forKey: "voice.enabled") {
        didSet { UserDefaults.standard.set(enabled, forKey: "voice.enabled"); if !enabled { stop() } }
    }
    @Published var status = ""
    @Published var speaking = false
    @Published private(set) var activeMessageID: UUID?
    let playback = ReadbackTransport()
    private var task: Task<Void, Never>?
    private var generation = UUID()
    func speak(_ text: String, messageID: UUID? = nil) {
        stop()
        activeMessageID = messageID
        speaking = true
        let current = generation
        task = Task {
            defer { if current == generation { ReadbackAudioSession.end(); speaking = false; activeMessageID = nil; task = nil } }
            do {
                try Task.checkCancellation()
                try ReadbackAudioSession.begin()
                status = "Kokoro readback"
                try await playback.read(text)
                try Task.checkCancellation()
                status = "Kokoro playback finished"
            } catch {
                if current == generation {
                    status = Task.isCancelled ? "Playback stopped" : "Kokoro playback failed: \(error.localizedDescription)"
                }
            }
        }
    }
    func stop() {
        let wasSpeaking = speaking
        generation = UUID(); task?.cancel(); task = nil; playback.stop(); speaking = false; activeMessageID = nil; status = wasSpeaking ? "Playback stopped" : ""
        if wasSpeaking { ReadbackAudioSession.end() }
    }
}