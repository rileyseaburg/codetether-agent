import XCTest
@testable import CodeTether

@MainActor
final class BackgroundReadbackTests: XCTestCase {
    func testBuiltAppDeclaresBackgroundAudio() {
        let modes = Bundle.main.object(forInfoDictionaryKey: "UIBackgroundModes") as? [String]
        XCTAssertEqual(modes, ["audio"])
    }
    func testBackgroundKeepsReadbackButStopsCaptureAndRecognition() async {
        let (loop, mic, transcriber, speaker) = makeLoop()
        await loop.start()
        loop.finishTurn(with: "Read this response")
        await loop.micTask?.value
        XCTAssertEqual(loop.phase, .speaking)
        loop.enterBackground()
        XCTAssertEqual(loop.phase, .speaking)
        XCTAssertFalse(mic.isRunning)
        XCTAssertNil(transcriber.onFinal)
        XCTAssertEqual(speaker.completions.count, 1, "Backgrounding must not cancel the speaker")
        speaker.completions.removeLast()()
        XCTAssertEqual(loop.phase, .idle)
        XCTAssertFalse(mic.isRunning)
        XCTAssertNil(transcriber.onFinal, "Never reopen the microphone in the background")
    }
    func testForegroundDuringReadbackRestartsCaptureOnlyAfterCompletion() async {
        let (loop, mic, _, speaker) = makeLoop()
        await loop.start()
        loop.finishTurn(with: "Read this response")
        await loop.micTask?.value
        loop.enterBackground()
        loop.enterForeground()
        XCTAssertEqual(loop.phase, .speaking)
        XCTAssertFalse(mic.isRunning)
        XCTAssertEqual(speaker.completions.count, 1)
        speaker.completions.removeLast()()
        await loop.micTask?.value
        XCTAssertEqual(loop.phase, .listening)
        XCTAssertTrue(mic.isRunning)
        loop.stop()
    }
}