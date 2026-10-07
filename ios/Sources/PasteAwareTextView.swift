import SwiftUI
import UIKit

/// UITextView subclass that intercepts pasted images and forwards them as attachments.
final class PasteAwareTextView: UITextView, UITextViewDelegate {
    var onImagePaste: ((UIImage) -> Void)?
    var onTextUpdate: ((String) -> Void)?
    var placeholder: String = "" {
        didSet { refreshPlaceholder() }
    }
    private let placeholderLabel = UILabel()

    override func canPerformAction(_ action: Selector, withSender sender: Any?) -> Bool {
        if action == #selector(paste(_:)), isEditable, onImagePaste != nil,
           UIPasteboard.general.hasImages { return true }
        return super.canPerformAction(action, withSender: sender)
    }

    override func paste(itemProviders: [NSItemProvider]) {
        guard itemProviders.contains(where: { $0.canLoadObject(ofClass: UIImage.self) }) else {
            super.paste(itemProviders: itemProviders)
            return
        }
        PasteImageAttachment.load(from: itemProviders) { [weak self] image in
            if let image { self?.onImagePaste?(image) }
        }
    }

    override func paste(_ sender: Any?) {
        if let image = PasteImageAttachment.extract(from: UIPasteboard.general) {
            onImagePaste?(image)
            return
        }
        super.paste(sender)
    }

    func textViewDidChange(_ textView: UITextView) {
        refreshPlaceholder()
        onTextUpdate?(textView.text)
    }

    func embedPlaceholder() {
        placeholderLabel.textColor = .placeholderText
        placeholderLabel.numberOfLines = 1
        placeholderLabel.font = font
        addSubview(placeholderLabel)
        refreshPlaceholder()
    }

    private func refreshPlaceholder() {
        placeholderLabel.text = text.isEmpty ? placeholder : ""
        placeholderLabel.frame = CGRect(x: 4, y: 12, width: bounds.width - 8, height: 20)
    }
}