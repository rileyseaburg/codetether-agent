import SwiftUI
import UniformTypeIdentifiers

/// SwiftUI wrapper for `PasteAwareTextView` with the composer's rounded style.
struct PasteAwareTextField: UIViewRepresentable {
    @Binding var text: String
    var onImagePaste: (UIImage) -> Void
    var accessibilityIdentifier: String = ""
    var placeholder: String = ""

    func sizeThatFits(_ proposal: ProposedViewSize, uiView view: PasteAwareTextView, context: Context) -> CGSize? {
        guard proposal.width ?? 0 > 0 else { return nil }
        return ComposerSizing.size(thatFits: view, proposedWidth: proposal.width!)
    }

    func makeUIView(context: Context) -> PasteAwareTextView {
        let view = PasteAwareTextView()
        view.font = .systemFont(ofSize: 17)
        view.pasteConfiguration = UIPasteConfiguration(acceptableTypeIdentifiers:
            [UTType.image.identifier, UTType.text.identifier])
        view.backgroundColor = .secondarySystemBackground
        view.layer.cornerRadius = 18
        view.textContainerInset = UIEdgeInsets(top: 12, left: 4, bottom: 12, right: 4)
        view.textContainer.lineFragmentPadding = 0
        view.delegate = view
        view.onImagePaste = onImagePaste
        view.onTextUpdate = { text = $0 }
        view.accessibilityIdentifier = accessibilityIdentifier
        view.isScrollEnabled = true
        view.alwaysBounceVertical = false
        view.placeholder = placeholder
        view.embedPlaceholder()
        return view
    }

    func updateUIView(_ view: PasteAwareTextView, context: Context) {
        if view.text != text { view.text = text }
        view.onImagePaste = onImagePaste
    }
}