import AVFoundation

/// Kokoro speaker for voice mode: streams `SpeechChunks` through the
/// server TTS client and plays them with `VoicePlayback`, which leaves
/// the shared loudspeaker audio session alone; recognition is paused
/// until the entire reply has finished playing.
@MainActor
final class KokoroSpeaker: ObservableObject, SpeakerContract {
    @Published private(set) var speaking = false
    private var task: Task<Void, Never>?
    private var generation = 0

    func speak(_ text: String, completion: @escaping () -> Void) {
        stop()
        speaking = true
        let current = generation
        task = Task {
            defer { if current == generation { speaking = false; completion() } }
            do {
                for chunk in SpeechChunks.split(text) {
                    try Task.checkCancellation()
                    let audio = try await SpeechClient().audio(text: chunk, voice: "af_heart")
                    try Task.checkCancellation()
                    try await VoicePlayback().play(audio)
                }
            } catch { /* cancelled or Kokoro fetch failed; loop resumes listening */ }
        }
    }

    func stop() {
        generation += 1; task?.cancel(); task = nil; speaking = false
    }
}