import Foundation

/// Kind of file a user attached to a chat message.
enum AttachmentKind {
    case image
    case document

    /// Detects the kind from magic bytes: `%PDF-` marks a document, anything else is treated as an image.
    static func detect(_ data: Data) -> AttachmentKind {
        data.starts(with: Array("%PDF-".utf8)) ? .document : .image
    }
}
