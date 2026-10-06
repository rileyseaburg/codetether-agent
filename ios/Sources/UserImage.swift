import UIKit

struct UserImage: Identifiable {
    let id = UUID()
    let data: Data
    var serverPath: String?
    static let maximumAttachments = 3
    static func prepare(_ data: Data) throws -> UserImage {
        guard data.count < 25 * 1024 * 1024, let image = UIImage(data: data) else { throw ClientError.invalidResponse }
        return try prepare(image)
    }
    static func prepare(_ image: UIImage) throws -> UserImage {
        guard image.size.width > 0, image.size.height > 0 else { throw ClientError.invalidResponse }
        let factor = min(1, 1280 / max(image.size.width, image.size.height))
        let size = CGSize(width: image.size.width * factor, height: image.size.height * factor)
        let format = UIGraphicsImageRendererFormat()
        format.scale = 1
        let resized = UIGraphicsImageRenderer(size: size, format: format).image { _ in
            image.draw(in: CGRect(origin: .zero, size: size))
        }
        guard let encoded = resized.jpegData(compressionQuality: 0.75), encoded.count <= 4 * 1024 * 1024 else {
            throw ClientError.invalidResponse
        }
        return UserImage(data: encoded)
    }
}

struct ImageUpload: Encodable { let data: String }
struct UploadedImage: Decodable { let path: String }