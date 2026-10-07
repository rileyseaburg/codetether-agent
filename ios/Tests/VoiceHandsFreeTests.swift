import XCTest
@testable import CodeTether

@MainActor
final class VoiceHandsFreeTests: XCTestCase {
    func testTwoSuccessiveTurnsSendAndResumeWithoutTapping() async {
        let (loop, mic, transcriber, speaker) = makeLoop()
        var time: TimeInterval = 0
        loop.silence.now = { time }; loop.silence.interval = 3
        await loop.start()
        for word in ["First question", "Second question"] {
            XCTAssertEqual(loop.phase, .listening)
            transcriber.hear(word)
            mic.emit(.speechEnd, probability: 0)
            XCTAssertEqual(loop.phase, .userSpeaking, "Brief VAD silence must not submit")
            time += 2.9; loop.silence.fireIfExpired()
            XCTAssertEqual(loop.phase, .userSpeaking)
            time += 0.2; loop.silence.fireIfExpired()
            await loop.micTask?.value
            XCTAssertEqual(loop.phase, .speaking)
            speaker.completions.removeLast()()
        }
        XCTAssertEqual(speaker.spoken, ["Answer", "Answer"])
        XCTAssertEqual(loop.phase, .listening)
        loop.stop()
    }

    func testStopCancelsPendingAutoSend() async {
        let (loop, _, transcriber, speaker) = makeLoop()
        var time: TimeInterval = 0
        loop.silence.now = { time }
        await loop.start()
        transcriber.hear("Do not send this")
        loop.stop()
        time = 20; loop.silence.fireIfExpired()
        XCTAssertNil(loop.micTask)
        XCTAssertEqual(loop.phase, .idle)
        XCTAssertTrue(speaker.spoken.isEmpty)
    }
}



