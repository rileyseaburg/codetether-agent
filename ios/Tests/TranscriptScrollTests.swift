import XCTest
import UIKit
@testable import CodeTether

@MainActor
final class TranscriptScrollTests: XCTestCase {
    func testInitialLayoutStreamingAndViewportResizeFollowBottom() async {
        let view = UIScrollView(frame: CGRect(x: 0, y: 0, width: 320, height: 400))
        view.contentSize = CGSize(width: 320, height: 1000)
        let controller = TranscriptScrollController()
        controller.connect(view)
        await settle()
        XCTAssertEqual(view.contentOffset.y, 600, accuracy: 1)
        view.contentSize.height = 1500
        await settle()
        XCTAssertEqual(view.contentOffset.y, 1100, accuracy: 1)
        view.frame.size.height = 300
        await settle()
        XCTAssertEqual(view.contentOffset.y, 1200, accuracy: 1)
        view.bounds.size.height = 500
        await settle()
        XCTAssertEqual(view.contentOffset.y, 1000, accuracy: 1)
        controller.disconnect()
    }

    func testPausedReaderIsNotPulledDownAndJumpResumes() async {
        let view = UIScrollView(frame: CGRect(x: 0, y: 0, width: 320, height: 400))
        view.contentSize = CGSize(width: 320, height: 1000)
        let controller = TranscriptScrollController()
        controller.connect(view)
        await settle()
        controller.following = false
        view.contentOffset.y = 200
        view.contentSize.height = 1500
        await settle()
        XCTAssertEqual(view.contentOffset.y, 200, accuracy: 1)
        controller.resume()
        XCTAssertTrue(controller.following)
        XCTAssertEqual(view.contentOffset.y, 1100, accuracy: 1)
        controller.disconnect()
        view.contentSize.height = 2000
        await settle()
        XCTAssertEqual(view.contentOffset.y, 1100, accuracy: 1)
    }

    private func settle() async { try? await Task.sleep(nanoseconds: 30_000_000) }
}