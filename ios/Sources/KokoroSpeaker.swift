import AVFoundation

/// Seekable Kokoro readback for voice mode. The microphone engine owns the
/// shared loudspeaker audio session; recognition stays paused during playback,
/// including while the user pauses or seeks the reply.
@MainActor
final class KokoroSpeaker: ObservableObject, SpeakerContract {
    @Published private(set) var speaking = false
    private var task: Task<Void, Never>?
    private var generation = 0
    let playback = ReadbackTransport()

    func speak(_ text: String, completion: @escaping () -> Void) {
        stop()
        speaking = true
        let current = generation
        task = Task {
            defer { if current == generation { speaking = false; completion() } }
            do {
                try await playback.read(text)
            } catch { /* cancelled or Kokoro fetch failed; loop resumes listening */ }
        }
    }

    func stop() {
        generation += 1; task?.cancel(); task = nil; playback.stop(); speaking = false
    }
}