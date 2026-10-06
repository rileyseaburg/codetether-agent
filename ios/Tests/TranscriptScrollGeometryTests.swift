import XCTest
import UIKit
@testable import CodeTether

@MainActor
final class TranscriptScrollGeometryTests: XCTestCase {
    func testShortContentAndInsetsDoNotOverscroll() {
        let view = UIScrollView(frame: CGRect(x: 0, y: 0, width: 320, height: 400))
        view.contentInset = UIEdgeInsets(top: 20, left: 0, bottom: 30, right: 0)
        view.contentSize = CGSize(width: 320, height: 50)
        XCTAssertEqual(TranscriptScrollGeometry.bottomOffset(view), -20)
        view.contentSize.height = 1000
        XCTAssertEqual(TranscriptScrollGeometry.bottomOffset(view), 630)
        view.contentOffset.y = 565
        XCTAssertFalse(TranscriptScrollGeometry.isNearBottom(view))
        view.contentOffset.y = 566
        XCTAssertTrue(TranscriptScrollGeometry.isNearBottom(view))
    }
}