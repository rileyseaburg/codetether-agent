import XCTest
@testable import CodeTether

@MainActor
final class ScreenReplyFlowTests: XCTestCase {
    func testManualReplyMeansQueuedNotInsertion() async {
        let network = ScreenRequestFixture(), model = network.model()
        model.replies.draft = "CT-IOS-CHECK"
        model.sendReply(); model.sendReply()
        await model.replies.task?.value
        XCTAssertEqual(network.replies, ["CT-IOS-CHECK"])
        XCTAssertTrue(network.questions.isEmpty)
        XCTAssertTrue(model.replies.notice?.contains("not confirmed as typed") == true)
    }
    func testUnsafeOrOverLimitTextDoesNotReachTransport() async {
        let network = ScreenRequestFixture(), model = network.model()
        for text in ["", " ", "a\nb", "a\tb", "a\u{2028}b", String(repeating: "😀", count: 1001)] {
            model.sendReply(text)
            await model.replies.task?.value
        }
        XCTAssertTrue(network.replies.isEmpty)
    }
    func testPauseBlocksManualTyping() async {
        let network = ScreenRequestFixture(), model = network.model(status: "paused")
        model.sendReply("CT-IOS-CHECK")
        await model.replies.task?.value
        XCTAssertFalse(model.canSendReply)
        XCTAssertTrue(network.replies.isEmpty)
    }
    func testFailurePreservesDraftAndNeverRetries() async {
        let network = ScreenRequestFixture(), model = network.model()
        network.failure = .http(409); model.replies.draft = "CT-IOS-CHECK"
        model.sendReply(); await model.replies.task?.value
        XCTAssertEqual(network.replies, ["CT-IOS-CHECK"])
        XCTAssertEqual(model.replies.draft, "CT-IOS-CHECK")
        XCTAssertTrue(model.replies.notice?.contains("No automatic retry") == true)
    }
}