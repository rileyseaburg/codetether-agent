import SwiftUI

/// Assembles the voice-mode object graph: loop + mic + transcriber +
/// speaker, bound to the shared `ChatModel`. Bind-then-start happens in
/// `.task` (after `onAppear`) so `loop.start()` never races a missing
/// collaborator.
struct VoiceModeTab: View {
    @ObservedObject var chat: ChatModel
    @Environment(\.scenePhase) private var scenePhase
    @State private var creatingSession = false
    @StateObject private var loop = VoiceLoop()
    @StateObject private var mic = VoiceMicEngine()
    @StateObject private var transcriber = VoiceTranscriber()
    @StateObject private var speaker = KokoroSpeaker()
    @State private var started = false

    var body: some View {
        VoiceSessionScreen(chat: chat, loop: loop, mic: mic, creating: $creatingSession)
            .task {
                guard !started else { return }
                started = true
                chat.voiceModeActive = true
                loop.attach(mic: mic, transcriber: transcriber, speaker: speaker,
                            send: { await chat.sendAsync($0) },
                            stopAgent: { chat.stop() })
                if chat.models.isEmpty { await chat.loadModels() }
                await loop.start()
            }
            .onChange(of: transcriber.error) { _, error in
                if let error { loop.stop(); loop.phase = .failed(error) }
            }
            .onChange(of: scenePhase) { _, phase in
                if phase == .background { loop.stop() }
            }
            .onDisappear { loop.stop(); chat.voiceModeActive = false; started = false }
    }
}