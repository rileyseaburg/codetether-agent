import XCTest
import UIKit
@testable import CodeTether

final class ImageThumbnailTests: XCTestCase {
    func testLargeImageIsDownsampledBeforeDisplay() throws {
        let format = UIGraphicsImageRendererFormat()
        format.scale = 1
        let image = UIGraphicsImageRenderer(size: CGSize(width: 2400, height: 1600), format: format).image { context in
            UIColor.blue.setFill()
            context.fill(CGRect(x: 0, y: 0, width: 2400, height: 1600))
        }
        let data = try XCTUnwrap(image.pngData())
        let thumbnail = try XCTUnwrap(ImageThumbnail.decode(data, maximumPixels: 256))
        XCTAssertLessThanOrEqual(try XCTUnwrap(thumbnail.cgImage).width, 256)
        XCTAssertLessThanOrEqual(try XCTUnwrap(thumbnail.cgImage).height, 256)
    }
    func testInvalidImageDoesNotCreateDisplayBuffer() {
        XCTAssertNil(ImageThumbnail.decode(Data("not an image".utf8)))
    }
}
