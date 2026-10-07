import UIKit
import UniformTypeIdentifiers
import XCTest
@testable import CodeTether

final class ScreenshotPasteTests: XCTestCase {
    @MainActor private func screenshot() -> UIImage {
        UIGraphicsImageRenderer(size: CGSize(width: 390, height: 844)).image { context in
            UIColor.blue.setFill()
            context.fill(CGRect(x: 0, y: 0, width: 390, height: 844))
        }
    }

    @MainActor func testImageOnlyClipboardEnablesPasteMenu() throws {
        let saved = UIPasteboard.general.items
        defer { UIPasteboard.general.items = saved }
        let image = screenshot()
        UIPasteboard.general.setData(try XCTUnwrap(image.pngData()), forPasteboardType: UTType.png.identifier)
        let view = PasteAwareTextView()
        var pasted: UIImage?
        view.onImagePaste = { pasted = $0 }
        XCTAssertTrue(view.canPerformAction(#selector(UIResponderStandardEditActions.paste(_:)), withSender: nil))
        view.paste(nil)
        XCTAssertEqual(pasted?.cgImage?.width, image.cgImage?.width)
        XCTAssertEqual(pasted?.cgImage?.height, image.cgImage?.height)
    }

    @MainActor func testNativePasteControlAcceptsPNGProviderAndAttachesImage() async throws {
        let target = ImagePasteTarget()
        let chat = ChatModel()
        let done = expectation(description: "Screenshot attachment")
        target.onImage = { image in chat.attachCapture(image); done.fulfill() }
        let png = try XCTUnwrap(screenshot().pngData())
        target.paste(itemProviders: [NSItemProvider(item: png as NSData, typeIdentifier: UTType.png.identifier)])
        await fulfillment(of: [done], timeout: 5)
        XCTAssertEqual(chat.attachments.count, 1)
        XCTAssertEqual(chat.attachments.first?.kind, .image)
    }

    @MainActor func testProviderPasteIntoTextFieldForwardsImage() async {
        let view = PasteAwareTextView()
        let done = expectation(description: "Text view image paste")
        view.onImagePaste = { image in
            XCTAssertEqual(image.size, CGSize(width: 390, height: 844))
            done.fulfill()
        }
        view.paste(itemProviders: [NSItemProvider(object: screenshot())])
        await fulfillment(of: [done], timeout: 5)
    }
}