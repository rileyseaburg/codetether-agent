import XCTest
@testable import CodeTether

final class ComposerSizingTests: XCTestCase {
    private func makeView() -> UITextView {
        let view = UITextView()
        view.font = .systemFont(ofSize: 17)
        view.textContainerInset = UIEdgeInsets(top: 12, left: 4, bottom: 12, right: 4)
        view.textContainer.lineFragmentPadding = 0
        return view
    }

    func testWidthNeverExceedsProposedWidth() {
        let view = makeView()
        view.text = String(repeating: "unbroken ", count: 80)
        let size = ComposerSizing.size(thatFits: view, proposedWidth: 300)
        XCTAssertLessThanOrEqual(size.width, 300)
        XCTAssertEqual(size.width, 300)
        XCTAssertGreaterThan(size.height, 12 + 12)
    }

    func testSingleLineStillFillsWidth() {
        let view = makeView()
        view.text = "hi"
        let size = ComposerSizing.size(thatFits: view, proposedWidth: 300)
        XCTAssertEqual(size.width, 300)
        XCTAssertEqual(size.height, 50, accuracy: 5)
    }
}
