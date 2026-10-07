import Testing
@testable import CodeTether

@MainActor
func makeLoop(sendReply: String? = "Answer") -> (VoiceLoop, FakeMic, FakeTranscriber, FakeSpeaker) {
    let loop = VoiceLoop()
    let mic = FakeMic(); let transcriber = FakeTranscriber(); let speaker = FakeSpeaker()
    loop.attach(mic: mic, transcriber: transcriber, speaker: speaker,
                send: { _ in sendReply }, stopAgent: {})
    return (loop, mic, transcriber, speaker)
}

@MainActor @Test func loopSendsTurnAndSpeaksReply() async throws {
    let (loop, mic, transcriber, speaker) = makeLoop()
    await loop.start()
    #expect(loop.phase == .listening)
    mic.emit(.speechStart, probability: 0.9)
    #expect(loop.phase == .userSpeaking)
    mic.emit(.speechEnd, probability: 0.1)
    loop.endTurnNow()   // Manual send remains available; hands-free has separate deadline tests.
    #expect(loop.phase == .thinking)
    try await Task.sleep(for: .seconds(1.2))   // settle + send
    #expect(loop.phase == .speaking)
    #expect(speaker.spoken == ["Answer"])
    speaker.completions.removeLast()()
    #expect(loop.phase == .listening)
    _ = transcriber
}

@MainActor @Test func stopDuringListeningDoesNotSend() async throws {
    let (loop, _, transcriber, speaker) = makeLoop()
    await loop.start()
    loop.stop()
    #expect(loop.phase == .idle)
    #expect(speaker.spoken.isEmpty)
    _ = transcriber
}

@MainActor @Test func stopPreventsStrayFinalsFromSending() async throws {
    let (loop, _, _, speaker) = makeLoop()
    await loop.start()
    loop.stop()
    loop.finishTurn(with: "ghost")                  // stray onFinal after stop
    try await Task.sleep(for: .seconds(0.1))
    #expect(loop.phase == .idle)
    #expect(speaker.spoken.isEmpty)
}