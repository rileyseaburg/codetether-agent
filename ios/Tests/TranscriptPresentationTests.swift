import XCTest
@testable import CodeTether

final class TranscriptPresentationTests: XCTestCase {
    func testRuntimeEnvelopeIsNotAUserChatMessage() {
        let machine = "Continue working toward the active thread goal.\n<objective>Test</objective>\nCompletion audit:"
        XCTAssertTrue(TranscriptPresentation.isRuntimeContinuation(machine))
        XCTAssertFalse(TranscriptPresentation.isRuntimeContinuation("Continue working toward the active thread goal."))
        XCTAssertFalse(TranscriptPresentation.isRuntimeContinuation("Please explain <objective> and Completion audit:"))
    }
    func testPrivateRuntimeFooterAndAttachmentPathsAreNotReadAloud() {
        XCTAssertEqual(TranscriptPresentation.displayText("Answer\n\nRuntime scope ledger: internal"), "Answer")
        XCTAssertEqual(TranscriptPresentation.displayText("Look\n\nUser attached image files: /private/path"), "Look")
    }
}
