import XCTest
@testable import CodeTether

@MainActor
final class ScreenMultiTurnTests: XCTestCase {
    func testTwentySequentialQuestionsAndRepliesKeepSameSession() async {
        let fixture = ScreenTurnFixture(), model = fixture.model()
        for turn in 1...20 {
            model.questions.draft = "Question \(turn)"
            model.askQuestion(); model.askQuestion()
            await model.questions.task?.value
            XCTAssertTrue(model.questions.waitingForFrame)
            XCTAssertFalse(model.canAskQuestion)
            fixture.emit(model, .capture, turn * 3, "analyzing")
            fixture.emit(model, .delta, turn * 3 + 1, nil)
            fixture.emit(model, .done, turn * 3 + 2, "ready")
            XCTAssertTrue(model.canAskQuestion)
            XCTAssertNil(model.questions.notice)
            model.replies.draft = "Manual reply \(turn)"
            model.sendReply(); model.sendReply()
            await model.replies.task?.value
            XCTAssertEqual(model.session?.id, fixture.receipt.id)
        }
        XCTAssertEqual(fixture.requests, Array(repeating: fixture.receipt.id, count: 20))
        XCTAssertEqual(fixture.replies, Array(repeating: fixture.receipt.id, count: 20))
    }
    func testTerminalEventBeforeHTTPReceiptDoesNotBlockNextTurn() async {
        for kind in [ScreenEvent.Kind.done, .error] {
            let fixture = ScreenTurnFixture(), model = fixture.model()
            fixture.duringAsk = { fixture.emit(model, kind, 2, kind == .done ? "ready" : "error") }
            model.questions.draft = "First question"
            model.askQuestion(); await model.questions.task?.value
            XCTAssertFalse(model.questions.waitingForFrame)
            XCTAssertNil(model.questions.notice)
            XCTAssertTrue(model.canAskQuestion)
            fixture.duringAsk = nil
            model.questions.draft = "Next question"
            model.askQuestion(); await model.questions.task?.value
            XCTAssertEqual(fixture.requests.count, 2)
            XCTAssertTrue(model.questions.waitingForFrame)
        }
    }
}
