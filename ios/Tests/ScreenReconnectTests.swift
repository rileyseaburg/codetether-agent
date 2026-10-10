import XCTest
@testable import CodeTether

@MainActor
final class ScreenReconnectTests: XCTestCase {
    func testManualReconnectAfterRetryExhaustionKeepsSessionAndDrafts() async {
        let fixture = ScreenTurnFixture(), model = fixture.model()
        fixture.failStreams = true
        model.questions.draft = "Unsent question"
        model.replies.draft = "Unsent reply"
        model.connect()
        await model.streamTask?.value
        XCTAssertEqual(fixture.streams, 5)
        XCTAssertTrue(model.retryBlocked)
        XCTAssertTrue(model.canReconnect)
        fixture.failStreams = false
        model.reconnect()
        for _ in 0..<100 where !model.connected { await Task.yield() }
        XCTAssertTrue(model.connected)
        XCTAssertFalse(model.retryBlocked)
        XCTAssertEqual(model.session?.id, fixture.receipt.id)
        XCTAssertEqual(model.questions.draft, "Unsent question")
        XCTAssertEqual(model.replies.draft, "Unsent reply")
        XCTAssertTrue(fixture.requests.isEmpty)
        XCTAssertTrue(fixture.replies.isEmpty)
        model.setActive(false)
        await model.streamTask?.value
    }
    func testDuplicateActivationDoesNotRestartHealthyStream() async {
        let fixture = ScreenTurnFixture(), model = fixture.model()
        model.connect()
        for _ in 0..<100 where !model.connected { await Task.yield() }
        let generation = model.generation
        model.setActive(true)
        XCTAssertEqual(model.generation, generation)
        model.setActive(false)
        await model.streamTask?.value
    }
}
