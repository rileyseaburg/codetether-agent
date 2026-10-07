import UIKit
import UniformTypeIdentifiers

/// Extracts image payloads from a `UIPasteboard` for chat attachment.
enum PasteImageAttachment {
    /// Returns the pasteboard image, if any, ready for `UserImage.prepare`.
    static func extract(from pasteboard: UIPasteboard) -> UIImage? {
        if let image = pasteboard.image { return image }
        for type in [UTType.png, .jpeg, .heic, .tiff] {
            if let data = pasteboard.data(forPasteboardType: type.identifier),
               let image = UIImage(data: data) { return image }
        }
        return nil
    }

    /// The native paste control provides authorized image providers, including screenshots.
    @MainActor
    static func load(from providers: [NSItemProvider], completion: @escaping (UIImage?) -> Void) {
        guard let provider = providers.first(where: { $0.canLoadObject(ofClass: UIImage.self) }) else {
            completion(nil)
            return
        }
        _ = provider.loadObject(ofClass: UIImage.self) { object, _ in
            DispatchQueue.main.async { completion(object as? UIImage) }
        }
    }
}