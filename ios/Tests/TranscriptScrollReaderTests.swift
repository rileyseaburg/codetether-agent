import Combine
import UIKit
import XCTest
@testable import CodeTether

@MainActor
final class TranscriptScrollReaderTests: XCTestCase {
    func testReaderUpdatesAreDeferredAndCoalesced() async {
        let view = UIScrollView(frame: CGRect(x: 0, y: 0, width: 320, height: 400))
        view.contentSize = CGSize(width: 320, height: 1000)
        let controller = TranscriptScrollController()
        controller.connect(view)
        await settle()
        var publications = 0
        let subscription = controller.$following.dropFirst().sink { _ in publications += 1 }
        view.contentOffset.y = 100
        controller.scheduleReaderUpdate(view)
        controller.scheduleReaderUpdate(view)
        XCTAssertTrue(controller.following, "Do not publish inside UIKit's KVO/layout stack")
        XCTAssertEqual(publications, 0)
        await settle()
        XCTAssertFalse(controller.following)
        XCTAssertEqual(publications, 1)
        controller.scheduleReaderUpdate(view)
        await settle()
        XCTAssertEqual(publications, 1, "Unchanged geometry must not trigger SwiftUI redraws")
        withExtendedLifetime(subscription) {}
        controller.disconnect()
    }

    func testPendingReaderUpdateCannotChangeReconnectedTranscript() async {
        let oldView = UIScrollView(frame: CGRect(x: 0, y: 0, width: 320, height: 400))
        oldView.contentSize = CGSize(width: 320, height: 1000)
        let controller = TranscriptScrollController()
        controller.connect(oldView)
        await settle()
        oldView.contentOffset.y = 100
        controller.scheduleReaderUpdate(oldView)
        controller.connect(UIScrollView())
        await settle()
        XCTAssertTrue(controller.following)
        controller.disconnect()
    }

    private func settle() async { try? await Task.sleep(nanoseconds: 30_000_000) }
}