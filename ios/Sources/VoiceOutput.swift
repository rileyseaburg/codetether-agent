import Foundation

@MainActor
final class VoiceOutput: ObservableObject {
    @Published var enabled = UserDefaults.standard.bool(forKey: "voice.enabled") {
        didSet { UserDefaults.standard.set(enabled, forKey: "voice.enabled"); if !enabled { stop() } }
    }
    @Published var status = ""
    @Published var speaking = false
    @Published private(set) var activeMessageID: UUID?
    private let playback = AudioPlayback()
    private var task: Task<Void, Never>?
    private var generation = UUID()
    func speak(_ text: String, messageID: UUID? = nil) {
        stop()
        activeMessageID = messageID
        speaking = true
        let current = generation
        task = Task {
            defer { if current == generation { speaking = false; activeMessageID = nil; task = nil } }
            do {
                for chunk in SpeechChunks.split(text) {
                    try Task.checkCancellation()
                    status = "Generating Kokoro audio…"
                    let audio = try await SpeechClient().audio(text: chunk, voice: "af_heart")
                    try Task.checkCancellation()
                    status = "Playing Kokoro audio"
                    try await playback.play(audio)
                }
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
    }
}
