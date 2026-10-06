import UIKit
import ImageIO

enum ImageThumbnail {
    static func decode(_ data: Data, maximumPixels: Int = 1024) -> UIImage? {
        guard let source = CGImageSourceCreateWithData(data as CFData,
            [kCGImageSourceShouldCache as String: false] as CFDictionary) else { return nil }
        let options: [String: CFTypeRef] = [
            kCGImageSourceCreateThumbnailFromImageAlways as String: kCFBooleanTrue,
            kCGImageSourceCreateThumbnailWithTransform as String: kCFBooleanTrue,
            kCGImageSourceShouldCacheImmediately as String: kCFBooleanTrue,
            kCGImageSourceThumbnailMaxPixelSize as String: NSNumber(value: maximumPixels)
        ]
        guard let image = CGImageSourceCreateThumbnailAtIndex(source, 0, options as CFDictionary) else { return nil }
        return UIImage(cgImage: image)
    }
}
