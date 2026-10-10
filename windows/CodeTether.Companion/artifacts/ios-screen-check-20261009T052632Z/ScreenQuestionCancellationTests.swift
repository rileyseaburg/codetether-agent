import XCTest
@testable import CodeTether

@MainActor
final class ScreenQuestionCancellationTests: XCTestCase {
    func testBackgroundRejectsLateReceiptAndRetainsDraft() async {
        let network = ScreenRequestFixture(), model = network.model()
        network.hold = true; model.questions.draft = "Type CT-IOS-CHECK"
        model.askQuestion(); let task = model.questions.task
        await network.waitForRequest()
        XCTAssertNotNil(network.pending)
        model.setActive(false)
        network.release(); await task?.value
        XCTAssertEqual(model.session?.id, network.receipt.id)
        XCTAssertEqual(model.questions.draft, "Type CT-IOS-CHECK")
        XCTAssertFalse(model.questions.submitting)
        XCTAssertTrue(model.questions.notice?.contains("interrupted") == true)
        XCTAssertEqual(network.questions.count, 1)
    }
    func testOldSessionReceiptCannotClearReplacementDraft() async {
        let network = ScreenRequestFixture(), model = network.model()
        network.hold = true; model.questions.draft = "Old draft"
        model.askQuestion(); let task = model.questions.task
        await network.waitForRequest()
        model.finishSession("Stopped")
        model.questions.draft = "Replacement draft"
        network.release(); await task?.value
        XCTAssertNil(model.session)
        XCTAssertEqual(model.questions.draft, "Replacement draft")
        XCTAssertNil(model.questions.notice)
    }
    func testEmptyAndOverLimitQuestionsNeverReachTransport() async {
        let network = ScreenRequestFixture(), model = network.model()
        for text in ["  \n", String(repeating: "😀", count: 1001)] {
            model.questions.draft = text; model.askQuestion()
            await model.questions.task?.value
        }
        XCTAssertTrue(network.questions.isEmpty)
        XCTAssertNotNil(model.questions.error)
    }
}