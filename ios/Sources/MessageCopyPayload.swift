import UIKit
import MarkdownUI
import UniformTypeIdentifiers

/// A single clipboard item with the requested representation and a plain-text fallback.
@MainActor
enum MessageCopyPayload {
    static func make(_ text: String, format: MessageCopyFormat) throws -> [String: Data] {
        if format == .markdown {
            return [UTType.utf8PlainText.identifier: Data(text.utf8),
                    "net.daringfireball.markdown": Data(text.utf8)]
        }
        let content = MarkdownContent(text)
        let plain = Data(content.renderPlainText().utf8)
        guard format == .richText else { return [UTType.utf8PlainText.identifier: plain] }
        let html = try RichMessageHTML.document(content.renderHTML())
        let htmlData = Data(html.utf8)
        let attributed = try NSAttributedString(data: htmlData, options: [
            .documentType: NSAttributedString.DocumentType.html,
            .characterEncoding: String.Encoding.utf8.rawValue
        ], documentAttributes: nil)
        let rtf = try attributed.data(from: NSRange(location: 0, length: attributed.length),
                                     documentAttributes: [.documentType: NSAttributedString.DocumentType.rtf])
        return [UTType.utf8PlainText.identifier: plain, UTType.html.identifier: htmlData,
                UTType.rtf.identifier: rtf]
    }
}

