import XCTest
@testable import CodeTether

@MainActor
final class VoiceDoubleReadbackTests: XCTestCase {
    func testNaturalFinalAndVADEndSpeakOnlyOnce() async {
        let (loop, mic, _, speaker) = makeLoop()
        var sends = 0
        loop.send = { _ in sends += 1; return "Answer" }
        await loop.start()
        loop.finishTurn(with: "Hello")
        XCTAssertEqual(loop.phase, .thinking)
        loop.finishTurn(with: "Hello")
        mic.emit(.speechEnd, probability: 0.1)
        await loop.micTask?.value
        XCTAssertEqual(sends, 1)
        XCTAssertEqual(speaker.spoken, ["Answer"])
        loop.stop()
    }

    func testInterruptedTurnCannotReadItsLateReply() async {
        let (loop, _, _, speaker) = makeLoop()
        var pending: CheckedContinuation<String?, Never>?
        let started = expectation(description: "Agent request started")
        loop.send = { _ in
            await withCheckedContinuation { pending = $0; started.fulfill() }
        }
        await loop.start()
        loop.finishTurn(with: "Hello")
        let task = loop.micTask
        await fulfillment(of: [started], timeout: 2)
        loop.tapped()
        pending?.resume(returning: "Old answer")
        await task?.value
        XCTAssertTrue(speaker.spoken.isEmpty)
        XCTAssertEqual(loop.phase, .listening)
        loop.stop()
    }
}

