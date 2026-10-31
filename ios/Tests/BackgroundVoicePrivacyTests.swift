import XCTest
@testable import CodeTether

@MainActor
final class BackgroundVoicePrivacyTests: XCTestCase {
    func testBackgroundListeningDoesNotSubmitOrRestartMicrophone() async {
        let (loop, mic, transcriber, speaker) = makeLoop()
        await loop.start()
        transcriber.hear("unfinished speech")
        loop.enterBackground()
        loop.finishTurn(with: "stale recognition callback")
        await loop.start()
        XCTAssertEqual(loop.phase, .idle)
        XCTAssertFalse(mic.isRunning)
        XCTAssertNil(transcriber.onFinal)
        XCTAssertTrue(speaker.spoken.isEmpty)
        loop.enterForeground()
        XCTAssertEqual(loop.phase, .idle)
        XCTAssertFalse(mic.isRunning)
        await loop.start()
        XCTAssertTrue(mic.isRunning)
        loop.stop()
    }
    func testExplicitStopStillCancelsBackgroundReadback() async {
        let (loop, mic, transcriber, speaker) = makeLoop()
        await loop.start()
        loop.finishTurn(with: "Read this response")
        await loop.micTask?.value
        loop.enterBackground()
        loop.stop()
        XCTAssertEqual(loop.phase, .idle)
        XCTAssertFalse(mic.isRunning)
        XCTAssertNil(transcriber.onFinal)
        XCTAssertTrue(speaker.completions.isEmpty)
        loop.enterForeground()
        XCTAssertEqual(loop.phase, .idle)
    }
}