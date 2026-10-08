import XCTest
import SwiftUI
@testable import CodeTether

@MainActor
final class WorkingIndicatorTests: XCTestCase {
    func testBusyAndIdleFootersHaveIdenticalMeasuredHeight() {
        let busy = UIHostingController(rootView: TranscriptWorkingIndicator(busy: true))
        let idle = UIHostingController(rootView: TranscriptWorkingIndicator(busy: false))
        let proposed = CGSize(width: 320, height: 1000)
        XCTAssertEqual(busy.sizeThatFits(in: proposed).height, 40, accuracy: 0.5)
        XCTAssertEqual(idle.sizeThatFits(in: proposed).height, 40, accuracy: 0.5)
    }

    func testServerSessionBindingDoesNotChangeConversationIdentity() {
        let chat = ChatModel()
        let identity = chat.conversationID
        chat.sessionID = UUID().uuidString
        XCTAssertEqual(chat.conversationID, identity)
        chat.clear()
        XCTAssertNotEqual(chat.conversationID, identity)
    }
}
