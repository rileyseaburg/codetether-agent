import XCTest
@testable import CodeTether

@MainActor
final class ScreenQuestionFlowTests: XCTestCase {
    func testExplicitTypingUsesFreshRequestOnceWithoutManualReply() async {
        let network = ScreenRequestFixture(), model = network.model()
        model.questions.draft = "Type CT-IOS-CHECK in the focused Notepad document. Do not submit."
        model.askQuestion(); model.askQuestion()
        await model.questions.task?.value
        XCTAssertEqual(network.questions.count, 1)
        XCTAssertTrue(network.questions[0].hasPrefix("Type CT-IOS-CHECK"))
        XCTAssertTrue(network.replies.isEmpty)
        XCTAssertEqual(model.questions.draft, "")
    }
    func testCaptureNowUsesStandingPromptAndKeepsQuestionDraft() async {
        let network = ScreenRequestFixture(), model = network.model()
        model.questions.draft = "Unsent draft"
        model.captureNow()
        await model.questions.task?.value
        XCTAssertEqual(network.questions, [model.prompt])
        XCTAssertEqual(model.questions.draft, "Unsent draft")
    }
    func testRejectedQuestionDoesNotRetryOrLoseSessionOrDraft() async {
        for code in [401, 403, 404, 409, 410, 429, 500] {
            let network = ScreenRequestFixture(), model = network.model()
            network.failure = .http(code)
            model.questions.draft = "Type CT-IOS-CHECK"
            model.askQuestion()
            await model.questions.task?.value
            XCTAssertEqual(network.questions.count, 1)
            XCTAssertNotNil(model.questions.error)
            XCTAssertEqual(model.session?.id, network.receipt.id)
            XCTAssertEqual(model.questions.draft, "Type CT-IOS-CHECK")
            XCTAssertTrue(model.questions.notice?.contains("No automatic retry") == true)
        }
    }
    func testCompletedStreamBeforeHTTPReplyDoesNotRestoreWaitingNotice() async {
        let network = ScreenRequestFixture(), model = network.model()
        network.hold = true; model.questions.draft = "Describe the focused input"
        model.askQuestion(); let task = model.questions.task
        await network.waitForRequest()
        network.event(model, type: .done, seq: 2)
        network.release(); await task?.value
        XCTAssertNil(model.questions.notice)
    }
}