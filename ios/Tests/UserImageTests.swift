import UIKit
import XCTest
@testable import CodeTether

final class UserImageTests: XCTestCase {
    func testInvalidAndOversizeInputsAreRejected() {
        XCTAssertThrowsError(try UserImage.prepare(Data("not an image".utf8)))
        XCTAssertThrowsError(try UserImage.prepare(Data(count: 25 * 1024 * 1024)))
        XCTAssertThrowsError(try UserImage.prepare(UIImage()))
    }
    func testSmallImagesAreNotUpscaled() throws {
        let image = UIGraphicsImageRenderer(size: CGSize(width: 32, height: 20)).image { _ in }
        let prepared = try UserImage.prepare(image)
        XCTAssertEqual(UIImage(data: prepared.data)?.size, CGSize(width: 32, height: 20))
    }
    func testGalleryAndCaptureUseSamePreparation() throws {
        let image = UIGraphicsImageRenderer(size: CGSize(width: 1600, height: 2400)).image { _ in }
        let captured = try UserImage.prepare(image)
        let gallery = try UserImage.prepare(XCTUnwrap(image.pngData()))
        XCTAssertEqual(UIImage(data: captured.data)?.size, UIImage(data: gallery.data)?.size)
        XCTAssertEqual(UIImage(data: captured.data)?.size.height, 1280)
    }
    @MainActor
    func testInvalidCaptureReportsErrorWithoutAddingAttachment() {
        let chat = ChatModel(client: stubClient())
        chat.attachCapture(UIImage())
        XCTAssertTrue(chat.attachments.isEmpty)
        XCTAssertNotNil(chat.error)
    }
}