import UIKit
import XCTest
@testable import CodeTether

final class CameraAttachmentTests: XCTestCase {
    @MainActor
    func testCaptureUsesBoundedJPEGPreparation() {
        let chat = ChatModel(client: stubClient())
        let image = UIGraphicsImageRenderer(size: CGSize(width: 2000, height: 1600)).image { _ in
            UIColor.blue.setFill()
            UIRectFill(CGRect(x: 0, y: 0, width: 2000, height: 1600))
        }
        chat.attachCapture(image)
        XCTAssertEqual(chat.attachments.count, 1)
        let data = chat.attachments[0].data
        XCTAssertEqual(Array(data.prefix(2)), [0xff, 0xd8])
        XCTAssertLessThanOrEqual(data.count, 4 * 1024 * 1024)
        XCTAssertEqual(UIImage(data: data)?.size.width, 1280)
        XCTAssertNil(chat.attachments[0].serverPath)
    }
    @MainActor
    func testCancellationPreservesDraftAndAttachments() {
        let chat = ChatModel(client: stubClient())
        chat.draft = "Keep my message"
        chat.addAttachment(UserImage(data: Data([1]), kind: .image))
        chat.attachCapture(nil)
        XCTAssertEqual(chat.draft, "Keep my message")
        XCTAssertEqual(chat.attachments.count, 1)
        XCTAssertNil(chat.error)
    }
    @MainActor
    func testBusyAndCapacityRejectFurtherAttachments() {
        let chat = ChatModel(client: stubClient())
        chat.busy = true
        chat.addAttachment(UserImage(data: Data([1]), kind: .image))
        chat.attachCapture(UIImage())
        XCTAssertTrue(chat.attachments.isEmpty)
        chat.busy = false
        for _ in 0...UserImage.maximumAttachments { chat.addAttachment(UserImage(data: Data([1]), kind: .image)) }
        chat.attachCapture(UIImage())
        XCTAssertEqual(chat.attachments.count, UserImage.maximumAttachments)
        XCTAssertNil(chat.error)
    }
}