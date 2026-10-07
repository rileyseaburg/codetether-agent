import UIKit

/// UIKit responder for UIPasteControl's authorized image providers.
final class ImagePasteTarget: UIView {
    var onImage: ((UIImage?) -> Void)?

    override init(frame: CGRect) {
        super.init(frame: frame)
        pasteConfiguration = UIPasteConfiguration(forAccepting: UIImage.self)
        let configuration = UIPasteControl.Configuration()
        configuration.displayMode = .iconOnly
        configuration.cornerStyle = .capsule
        let control = UIPasteControl(configuration: configuration)
        control.target = self
        control.accessibilityLabel = "Paste image"
        control.accessibilityIdentifier = "paste-image"
        control.translatesAutoresizingMaskIntoConstraints = false
        addSubview(control)
        NSLayoutConstraint.activate([
            control.centerXAnchor.constraint(equalTo: centerXAnchor),
            control.centerYAnchor.constraint(equalTo: centerYAnchor)
        ])
    }

    required init?(coder: NSCoder) { nil }

    override func paste(itemProviders: [NSItemProvider]) {
        let deliver = onImage
        PasteImageAttachment.load(from: itemProviders) { deliver?($0) }
    }
}