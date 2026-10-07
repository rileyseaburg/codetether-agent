import Foundation

/// Clipboard representations offered for the full, untruncated message.
enum MessageCopyFormat: String, CaseIterable, Identifiable {
    case richText = "Rich text"
    case plainText = "Plain text"
    case markdown = "Markdown"
    var id: Self { self }
}