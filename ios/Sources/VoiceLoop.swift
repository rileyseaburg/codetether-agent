import Foundation

/// Coordinates the full voice loop: mic → VAD → Apple Speech → agent →
/// Kokoro TTS → back to listening. The mic session stays active while
/// recognition pauses during replies; VAD events are phase-gated in the
/// events extension; turn actions live in `VoiceLoop+Turns`.
@MainActor
final class VoiceLoop: ObservableObject {
    // Internal setters: the turn-handling extension and voice tab bridge
    // (different files) mutate these; test fakes drive them too.
    @Published var phase: VoicePhase = .idle
    @Published var transcript = ""
    @Published var reply: String?
    @Published var probability: Float = 0
    @Published var vadAvailable = true
    var mic: MicEngineContract?
    var transcriber: TranscriberContract?
    var speaker: SpeakerContract?
    var send: ((String) async -> String?)?
    var stopAgent: (() -> Void)?
    var micTask: Task<Void, Never>?
    var turnID = UUID()
    var pendingFinal = false
    var speakStartedAt = Date()
    let silence = VoiceSilenceTimer()
    @Published var pauseSeconds = UserDefaults.standard.integer(forKey: "voice.pauseSeconds") == 5 ? 5 : 3 {
        didSet { silence.interval = Double(pauseSeconds); UserDefaults.standard.set(pauseSeconds, forKey: "voice.pauseSeconds") }
    }

    /// Binds collaborators; safe to re-bind between sessions.
    func attach(mic: MicEngineContract, transcriber: TranscriberContract,
                speaker: SpeakerContract, send: @escaping (String) async -> String?,
                stopAgent: @escaping () -> Void) {
        self.mic = mic; self.transcriber = transcriber; self.speaker = speaker
        self.send = send; self.stopAgent = stopAgent
        self.transcriber?.onUpdate = { [weak self] text in self?.showPartial(text) }
        silence.interval = Double(pauseSeconds)
        silence.onExpired = { [weak self] in self?.endTurnNow() }
    }

    /// Enters the loop: permissions → mic on → listening.
    func start() async {
        guard canStart, let mic else { return }
        let startup = turnID
        guard await transcriber?.requestPermissions() ?? false else {
            phase = .failed("Enable Microphone and Speech Recognition in Settings."); return
        }
        guard !Task.isCancelled, startup == turnID else { return }
        let vadReady = mic.prepare()
        do {
            try mic.start(onEvent: handle) { [weak self] buffer in
                Task { @MainActor in self?.appendListeningAudio(buffer) }
            }
            vadAvailable = vadReady
            phase = .listening
            resumeListening()
        } catch {
            phase = .failed("Microphone could not start.")
        }
    }
}