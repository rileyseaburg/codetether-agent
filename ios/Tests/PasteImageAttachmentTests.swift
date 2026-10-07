import UIKit
import XCTest
@testable import CodeTether

final class PasteImageAttachmentTests: XCTestCase {
    @MainActor
    func testPasteForwardsImageAndAttachmentPipelineEncodesJPEG() {
        let chat = ChatModel()
        let image = UIGraphicsImageRenderer(size: CGSize(width: 900, height: 700)).image { _ in
            UIColor.red.setFill()
            UIRectFill(CGRect(x: 0, y: 0, width: 900, height: 700))
        }
        let view = PasteAwareTextView()
        var pasted: UIImage?
        view.onImagePaste = { pasted = $0 }
        UIPasteboard.general.image = image
        view.paste(nil)
        XCTAssertNotNil(pasted, "image paste should forward the image")
        chat.attachCapture(pasted)
        XCTAssertEqual(chat.attachments.count, 1)
        XCTAssertEqual(Array(chat.attachments[0].data.prefix(2)), [0xff, 0xd8], "attachment should be JPEG")
        UIPasteboard.general.image = nil
    }

    @MainActor
    func testTextOnlyPasteDoesNotForwardImage() {
        let view = PasteAwareTextView()
        var pasted: UIImage?
        view.onImagePaste = { pasted = $0 }
        UIPasteboard.general.string = "hello"
        view.paste(nil)
        XCTAssertNil(pasted, "text paste should fall through to normal paste")
        UIPasteboard.general.string = nil
    }

    @MainActor
    func testExtractReturnsNilWithoutImage() {
        UIPasteboard.general.string = "words only"
        XCTAssertNil(PasteImageAttachment.extract(from: UIPasteboard.general))
        UIPasteboard.general.string = nil
    }
}
