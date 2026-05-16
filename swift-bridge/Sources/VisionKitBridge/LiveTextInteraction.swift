import AppKit
import Foundation
import VisionKit

struct VKPointPayload: Codable {
    var x: Double
    var y: Double

    init(point: CGPoint) {
        x = point.x
        y = point.y
    }

    var cgPoint: CGPoint {
        CGPoint(x: x, y: y)
    }
}

struct VKRectPayload: Codable {
    var x: Double
    var y: Double
    var width: Double
    var height: Double

    init(rect: CGRect) {
        x = rect.origin.x
        y = rect.origin.y
        width = rect.size.width
        height = rect.size.height
    }

    var cgRect: CGRect {
        CGRect(x: x, y: y, width: width, height: height)
    }
}

struct VKTextRangePayload: Codable {
    var location: Int
    var length: Int
}

struct VKAttributedTextAttributePayload: Codable {
    var name: String
    var value: String
}

struct VKAttributedTextRunPayload: Codable {
    var range: VKTextRangePayload
    var attributes: [VKAttributedTextAttributePayload]
}

struct VKAttributedTextPayload: Codable {
    var text: String
    var runs: [VKAttributedTextRunPayload]
}

struct VKMenuItemPayload: Codable {
    var title: String
    var tag: Int
    var isSeparator: Bool
    var isEnabled: Bool
    var isHidden: Bool
    var state: Int
    var submenu: VKMenuPayload?
}

struct VKMenuPayload: Codable {
    var title: String
    var items: [VKMenuItemPayload]
}

struct VKEventInfoPayload: Codable {
    var typeName: String
    var locationInWindow: VKPointPayload
    var modifierFlags: UInt64
    var keyCode: UInt16
    var characters: String?
    var charactersIgnoringModifiers: String?
    var clickCount: Int
}

struct VKDelegateConfigPayload: Codable {
    var shouldBegin: Bool = true
    var contentsRect: VKRectPayload?
    var shouldHandleKeyDownEvent: Bool = true
    var shouldShowMenuForEvent: Bool = true
    var updatedMenu: VKMenuPayload?
}

struct VKDelegateEventPayload: Codable {
    var kind: String
    var point: VKPointPayload?
    var analysisTypeRaw: UInt64?
    var decision: Bool?
    var rect: VKRectPayload?
    var event: VKEventInfoPayload?
    var menu: VKMenuPayload?
    var menuItem: VKMenuItemPayload?
    var visible: Bool?
    var highlighted: Bool?
    var hasContentView: Bool?
}

struct VKMenuTagsPayload: Codable {
    var copyImage: Int
    var shareImage: Int
    var copySubject: Int
    var shareSubject: Int
    var lookupItem: Int
    var recommendedAppItems: Int
}

struct VKFontPayload: Codable {
    var name: String
    var pointSize: Double
}

@available(macOS 13.0, *)
@MainActor
final class VKLiveTextContentViewBox: NSObject {
    let view: NSView

    init(frame: CGRect = .zero) {
        view = NSView(frame: frame)
        super.init()
    }
}

@available(macOS 13.0, *)
func vkLiveTextContentViewBox(
    _ token: UnsafeMutableRawPointer?
) throws -> VKLiveTextContentViewBox {
    guard let token else {
        throw VKBridgeError.invalidArgument("missing live text content view token")
    }
    return vkBorrow(token)
}

@available(macOS 13.0, *)
@MainActor
final class VKLiveTextTrackingImageViewBox: NSObject {
    let imageView: NSImageView

    init(frame: CGRect = .zero) {
        imageView = NSImageView(frame: frame)
        imageView.imageScaling = .scaleProportionallyUpOrDown
        super.init()
    }
}

@available(macOS 13.0, *)
func vkLiveTextTrackingImageViewBox(
    _ token: UnsafeMutableRawPointer?
) throws -> VKLiveTextTrackingImageViewBox {
    guard let token else {
        throw VKBridgeError.invalidArgument("missing live text tracking image view token")
    }
    return vkBorrow(token)
}

@available(macOS 13.0, *)
@MainActor
final class VKLiveTextInteractionDelegateBox: NSObject, ImageAnalysisOverlayViewDelegate {
    var config = VKDelegateConfigPayload()
    var contentViewBox: VKLiveTextContentViewBox?
    private(set) var recordedEvents: [VKDelegateEventPayload] = []

    private func record(_ event: VKDelegateEventPayload) {
        recordedEvents.append(event)
    }

    func clearRecordedEvents() {
        recordedEvents.removeAll(keepingCapacity: false)
    }

    func overlayView(
        _ overlayView: ImageAnalysisOverlayView,
        shouldBeginAt point: CGPoint,
        forAnalysisType analysisType: ImageAnalysisOverlayView.InteractionTypes
    ) -> Bool {
        let decision = config.shouldBegin
        record(
            VKDelegateEventPayload(
                kind: "shouldBeginAt",
                point: VKPointPayload(point: point),
                analysisTypeRaw: UInt64(analysisType.rawValue),
                decision: decision
            )
        )
        return decision
    }

    func contentsRect(for overlayView: ImageAnalysisOverlayView) -> CGRect {
        let rect = config.contentsRect?.cgRect ?? overlayView.bounds
        record(
            VKDelegateEventPayload(
                kind: "contentsRect",
                rect: VKRectPayload(rect: rect)
            )
        )
        return rect
    }

    #if compiler(>=5.3) && $NonescapableTypes
    func contentView(for overlayView: ImageAnalysisOverlayView) -> NSView? {
        let hasContentView = contentViewBox != nil
        record(
            VKDelegateEventPayload(
                kind: "contentView",
                hasContentView: hasContentView
            )
        )
        return contentViewBox?.view
    }
    #endif

    func overlayView(
        _ overlayView: ImageAnalysisOverlayView,
        shouldHandleKeyDownEvent event: NSEvent
    ) -> Bool {
        let decision = config.shouldHandleKeyDownEvent
        record(
            VKDelegateEventPayload(
                kind: "shouldHandleKeyDownEvent",
                decision: decision,
                event: vkEventInfoPayload(from: event)
            )
        )
        return decision
    }

    func overlayView(
        _ overlayView: ImageAnalysisOverlayView,
        shouldShowMenuForEvent event: NSEvent,
        atPoint point: CGPoint
    ) -> Bool {
        let decision = config.shouldShowMenuForEvent
        record(
            VKDelegateEventPayload(
                kind: "shouldShowMenuForEvent",
                point: VKPointPayload(point: point),
                decision: decision,
                event: vkEventInfoPayload(from: event)
            )
        )
        return decision
    }

    func overlayView(
        _ overlayView: ImageAnalysisOverlayView,
        liveTextButtonDidChangeToVisible visible: Bool
    ) {
        record(
            VKDelegateEventPayload(
                kind: "liveTextButtonDidChangeToVisible",
                visible: visible
            )
        )
    }

    func overlayView(
        _ overlayView: ImageAnalysisOverlayView,
        highlightSelectedItemsDidChange highlightSelectedItems: Bool
    ) {
        record(
            VKDelegateEventPayload(
                kind: "highlightSelectedItemsDidChange",
                highlighted: highlightSelectedItems
            )
        )
    }

    @available(macOS 14.0, *)
    func textSelectionDidChange(_ overlayView: ImageAnalysisOverlayView) {
        record(VKDelegateEventPayload(kind: "textSelectionDidChange"))
    }

    @available(macOS 14.0, *)
    func overlayView(
        _ overlayView: ImageAnalysisOverlayView,
        updatedMenuFor menu: NSMenu,
        for event: NSEvent,
        at point: CGPoint
    ) -> NSMenu {
        let updatedMenu = config.updatedMenu.map(vkMenu(from:)) ?? menu
        record(
            VKDelegateEventPayload(
                kind: "updatedMenuFor",
                point: VKPointPayload(point: point),
                event: vkEventInfoPayload(from: event),
                menu: vkMenuPayload(from: updatedMenu)
            )
        )
        return updatedMenu
    }

    @available(macOS 14.0, *)
    func overlayView(
        _ overlayView: ImageAnalysisOverlayView,
        needsUpdate menu: NSMenu
    ) {
        record(
            VKDelegateEventPayload(
                kind: "needsUpdate",
                menu: vkMenuPayload(from: menu)
            )
        )
    }

    @available(macOS 14.0, *)
    func overlayView(
        _ overlayView: ImageAnalysisOverlayView,
        willOpen menu: NSMenu
    ) {
        record(
            VKDelegateEventPayload(
                kind: "willOpen",
                menu: vkMenuPayload(from: menu)
            )
        )
    }

    @available(macOS 14.0, *)
    func overlayView(
        _ overlayView: ImageAnalysisOverlayView,
        didClose menu: NSMenu
    ) {
        record(
            VKDelegateEventPayload(
                kind: "didClose",
                menu: vkMenuPayload(from: menu)
            )
        )
    }

    #if compiler(>=5.3) && $NonescapableTypes
    @available(macOS 14.0, *)
    func overlayView(
        _ overlayView: ImageAnalysisOverlayView,
        menu: NSMenu,
        willHighlight menuItem: NSMenuItem?
    ) {
        record(
            VKDelegateEventPayload(
                kind: "menuWillHighlight",
                menu: vkMenuPayload(from: menu),
                menuItem: menuItem.map(vkMenuItemPayload(from:))
            )
        )
    }
    #endif
}

@available(macOS 13.0, *)
func vkLiveTextInteractionDelegateBox(
    _ token: UnsafeMutableRawPointer?
) throws -> VKLiveTextInteractionDelegateBox {
    guard let token else {
        throw VKBridgeError.invalidArgument(
            "missing live text interaction delegate token"
        )
    }
    return vkBorrow(token)
}

@available(macOS 13.0, *)
final class VKLiveTextSubjectBox: NSObject {
    let subject: ImageAnalysisOverlayView.Subject

    init(subject: ImageAnalysisOverlayView.Subject) {
        self.subject = subject
        super.init()
    }
}

@available(macOS 13.0, *)
func vkLiveTextSubjectBox(
    _ token: UnsafeMutableRawPointer?
) throws -> VKLiveTextSubjectBox {
    guard let token else {
        throw VKBridgeError.invalidArgument("missing live text subject token")
    }
    return vkBorrow(token)
}

@available(macOS 13.0, *)
@MainActor
final class VKLiveTextInteractionBox: NSObject {
    let overlayView: ImageAnalysisOverlayView
    var trackingImageViewBox: VKLiveTextTrackingImageViewBox?
    var delegateBox: VKLiveTextInteractionDelegateBox?

    override init() {
        overlayView = ImageAnalysisOverlayView(frame: .zero)
        trackingImageViewBox = VKLiveTextTrackingImageViewBox()
        super.init()
        attachTrackingImageView()
    }

    init(delegateBox: VKLiveTextInteractionDelegateBox) {
        overlayView = ImageAnalysisOverlayView(delegateBox)
        trackingImageViewBox = VKLiveTextTrackingImageViewBox()
        self.delegateBox = delegateBox
        super.init()
        overlayView.delegate = delegateBox
        attachTrackingImageView()
    }

    func ensureTrackingImageViewBox() -> VKLiveTextTrackingImageViewBox {
        if let trackingImageViewBox {
            return trackingImageViewBox
        }
        let trackingImageViewBox = VKLiveTextTrackingImageViewBox()
        self.trackingImageViewBox = trackingImageViewBox
        attachTrackingImageView()
        return trackingImageViewBox
    }

    func attachTrackingImageView() {
        #if compiler(>=5.3) && $NonescapableTypes
        overlayView.trackingImageView = trackingImageViewBox?.imageView
        #endif
    }
}

@available(macOS 13.0, *)
func vkLiveTextInteractionBox(
    _ token: UnsafeMutableRawPointer?
) throws -> VKLiveTextInteractionBox {
    guard let token else {
        throw VKBridgeError.invalidArgument(
            "missing live text interaction token"
        )
    }
    return vkBorrow(token)
}

@available(macOS 13.0, *)
func vkLiveTextInteractionTypes(
    from raw: UInt64
) -> ImageAnalysisOverlayView.InteractionTypes {
    ImageAnalysisOverlayView.InteractionTypes(rawValue: UInt(raw))
}

func vkTokenUInt64(_ token: UnsafeMutableRawPointer) -> UInt64 {
    UInt64(UInt(bitPattern: token))
}

func vkTokenPointer(_ rawValue: UInt64) -> UnsafeMutableRawPointer? {
    UnsafeMutableRawPointer(bitPattern: UInt(rawValue))
}

func vkTextRangePayload(
    from range: Range<String.Index>,
    in text: String
) -> VKTextRangePayload {
    let nsRange = NSRange(range, in: text)
    return VKTextRangePayload(location: nsRange.location, length: nsRange.length)
}

func vkStringRange(
    from payload: VKTextRangePayload,
    in text: String
) throws -> Range<String.Index> {
    let nsRange = NSRange(location: payload.location, length: payload.length)
    guard let range = Range(nsRange, in: text) else {
        throw VKBridgeError.invalidArgument(
            "invalid selected range (location=\(payload.location), length=\(payload.length))"
        )
    }
    return range
}

func vkAttributedTextPayload(from attributedText: AttributedString) -> VKAttributedTextPayload {
    let nsAttributedText = NSAttributedString(attributedText)
    var runs: [VKAttributedTextRunPayload] = []
    let fullRange = NSRange(location: 0, length: nsAttributedText.length)
    nsAttributedText.enumerateAttributes(in: fullRange, options: []) { attributes, range, _ in
        let payloadAttributes = attributes.map { key, value in
            VKAttributedTextAttributePayload(
                name: key.rawValue,
                value: String(describing: value)
            )
        }
        .sorted { $0.name < $1.name }
        runs.append(
            VKAttributedTextRunPayload(
                range: VKTextRangePayload(location: range.location, length: range.length),
                attributes: payloadAttributes
            )
        )
    }
    return VKAttributedTextPayload(text: nsAttributedText.string, runs: runs)
}

func vkMenuItemPayload(from item: NSMenuItem) -> VKMenuItemPayload {
    VKMenuItemPayload(
        title: item.title,
        tag: item.tag,
        isSeparator: item.isSeparatorItem,
        isEnabled: item.isEnabled,
        isHidden: item.isHidden,
        state: item.state.rawValue,
        submenu: item.submenu.map(vkMenuPayload(from:))
    )
}

func vkMenuPayload(from menu: NSMenu) -> VKMenuPayload {
    VKMenuPayload(title: menu.title, items: menu.items.map(vkMenuItemPayload(from:)))
}

func vkMenuItem(from payload: VKMenuItemPayload) -> NSMenuItem {
    let item = payload.isSeparator ? NSMenuItem.separator() : NSMenuItem(
        title: payload.title,
        action: nil,
        keyEquivalent: ""
    )
    item.tag = payload.tag
    item.isEnabled = payload.isEnabled
    item.isHidden = payload.isHidden
    item.state = NSControl.StateValue(rawValue: payload.state)
    if let submenu = payload.submenu {
        item.submenu = vkMenu(from: submenu)
    }
    return item
}

func vkMenu(from payload: VKMenuPayload) -> NSMenu {
    let menu = NSMenu(title: payload.title)
    for item in payload.items {
        menu.addItem(vkMenuItem(from: item))
    }
    return menu
}

func vkEventInfoPayload(from event: NSEvent) -> VKEventInfoPayload {
    VKEventInfoPayload(
        typeName: String(describing: event.type),
        locationInWindow: VKPointPayload(point: event.locationInWindow),
        modifierFlags: UInt64(event.modifierFlags.rawValue),
        keyCode: event.keyCode,
        characters: event.characters,
        charactersIgnoringModifiers: event.charactersIgnoringModifiers,
        clickCount: event.clickCount
    )
}

func vkFontPayload(from font: NSFont) -> VKFontPayload {
    VKFontPayload(name: font.fontName, pointSize: font.pointSize)
}

func vkFont(from payload: VKFontPayload) throws -> NSFont {
    guard let font = NSFont(name: payload.name, size: payload.pointSize) else {
        throw VKBridgeError.invalidArgument(
            "failed to create NSFont named '\(payload.name)' at size \(payload.pointSize)"
        )
    }
    return font
}

@available(macOS 13.0, *)
func vkSubjectTokens(
    from subjects: Set<ImageAnalysisOverlayView.Subject>
) -> [UInt64] {
    subjects.map { subject in
        vkTokenUInt64(vkRetain(VKLiveTextSubjectBox(subject: subject)))
    }
}

@available(macOS 13.0, *)
func vkSubjects(
    from cString: UnsafePointer<CChar>?
) throws -> Set<ImageAnalysisOverlayView.Subject> {
    let tokens = try vkDecodeJSON(cString, as: [UInt64].self)
    let subjects = try tokens.map { token -> ImageAnalysisOverlayView.Subject in
        guard let pointer = vkTokenPointer(token) else {
            throw VKBridgeError.invalidArgument("invalid live text subject token: \(token)")
        }
        return try vkLiveTextSubjectBox(pointer).subject
    }
    return Set(subjects)
}

func vkPNGData(from image: NSImage) throws -> Data {
    guard let tiffData = image.tiffRepresentation,
          let bitmap = NSBitmapImageRep(data: tiffData),
          let pngData = bitmap.representation(using: .png, properties: [:])
    else {
        throw VKBridgeError.framework("failed to encode NSImage as PNG")
    }
    return pngData
}

func vkWritePNGImage(
    _ image: NSImage,
    to outBytes: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    length outLength: UnsafeMutablePointer<UInt64>,
    width outWidth: UnsafeMutablePointer<Double>,
    height outHeight: UnsafeMutablePointer<Double>
) throws {
    let pngData = try vkPNGData(from: image)
    outWidth.pointee = image.size.width
    outHeight.pointee = image.size.height
    try vkWriteBytes(pngData, to: outBytes, length: outLength)
}

@_cdecl("vk_live_text_content_view_new")
public func vk_live_text_content_view_new() -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else {
        return nil
    }
    return try? vkOnMainActor {
        vkRetain(VKLiveTextContentViewBox())
    }
}

@_cdecl("vk_live_text_content_view_release")
public func vk_live_text_content_view_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    vkRelease(token)
}

@_cdecl("vk_live_text_content_view_frame")
public func vk_live_text_content_view_frame(
    _ token: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>,
    _ outY: UnsafeMutablePointer<Double>,
    _ outWidth: UnsafeMutablePointer<Double>,
    _ outHeight: UnsafeMutablePointer<Double>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextContentView requires macOS 13+"
            )
        }
        let box = try vkLiveTextContentViewBox(token)
        let frame = try vkOnMainActor { box.view.frame }
        outX.pointee = frame.origin.x
        outY.pointee = frame.origin.y
        outWidth.pointee = frame.size.width
        outHeight.pointee = frame.size.height
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_content_view_set_frame")
public func vk_live_text_content_view_set_frame(
    _ token: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextContentView requires macOS 13+"
            )
        }
        let box = try vkLiveTextContentViewBox(token)
        try vkOnMainActor {
            box.view.frame = CGRect(x: x, y: y, width: width, height: height)
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_tracking_image_view_new")
public func vk_live_text_tracking_image_view_new() -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else {
        return nil
    }
    return try? vkOnMainActor {
        vkRetain(VKLiveTextTrackingImageViewBox())
    }
}

@_cdecl("vk_live_text_tracking_image_view_release")
public func vk_live_text_tracking_image_view_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    vkRelease(token)
}

@_cdecl("vk_live_text_tracking_image_view_frame")
public func vk_live_text_tracking_image_view_frame(
    _ token: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>,
    _ outY: UnsafeMutablePointer<Double>,
    _ outWidth: UnsafeMutablePointer<Double>,
    _ outHeight: UnsafeMutablePointer<Double>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextTrackingImageView requires macOS 13+"
            )
        }
        let box = try vkLiveTextTrackingImageViewBox(token)
        let frame = try vkOnMainActor { box.imageView.frame }
        outX.pointee = frame.origin.x
        outY.pointee = frame.origin.y
        outWidth.pointee = frame.size.width
        outHeight.pointee = frame.size.height
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_tracking_image_view_set_frame")
public func vk_live_text_tracking_image_view_set_frame(
    _ token: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextTrackingImageView requires macOS 13+"
            )
        }
        let box = try vkLiveTextTrackingImageViewBox(token)
        try vkOnMainActor {
            box.imageView.frame = CGRect(x: x, y: y, width: width, height: height)
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_tracking_image_view_set_image_at_path")
public func vk_live_text_tracking_image_view_set_image_at_path(
    _ token: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextTrackingImageView requires macOS 13+"
            )
        }
        let box = try vkLiveTextTrackingImageViewBox(token)
        let path = try vkRequireFilePath(path, field: "path")
        let image = try vkLoadNSImage(at: path)
        try vkOnMainActor {
            box.imageView.image = image
            box.imageView.frame = CGRect(origin: .zero, size: image.size)
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_tracking_image_view_image_size")
public func vk_live_text_tracking_image_view_image_size(
    _ token: UnsafeMutableRawPointer?,
    _ outHasImage: UnsafeMutablePointer<Int32>,
    _ outWidth: UnsafeMutablePointer<Double>,
    _ outHeight: UnsafeMutablePointer<Double>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextTrackingImageView requires macOS 13+"
            )
        }
        let box = try vkLiveTextTrackingImageViewBox(token)
        let image = try vkOnMainActor { box.imageView.image }
        if let image {
            outHasImage.pointee = 1
            outWidth.pointee = image.size.width
            outHeight.pointee = image.size.height
        } else {
            outHasImage.pointee = 0
            outWidth.pointee = 0
            outHeight.pointee = 0
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_delegate_new")
public func vk_live_text_interaction_delegate_new() -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else {
        return nil
    }
    return try? vkOnMainActor {
        vkRetain(VKLiveTextInteractionDelegateBox())
    }
}

@_cdecl("vk_live_text_interaction_delegate_release")
public func vk_live_text_interaction_delegate_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    vkRelease(token)
}

@_cdecl("vk_live_text_interaction_delegate_config_json")
public func vk_live_text_interaction_delegate_config_json(
    _ token: UnsafeMutableRawPointer?,
    _ outConfigJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteractionDelegate requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionDelegateBox(token)
        try vkWriteJSON(try vkOnMainActor { box.config }, to: outConfigJson)
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_delegate_set_config_json")
public func vk_live_text_interaction_delegate_set_config_json(
    _ token: UnsafeMutableRawPointer?,
    _ configJson: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteractionDelegate requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionDelegateBox(token)
        let config = try vkDecodeJSON(configJson, as: VKDelegateConfigPayload.self)
        try vkOnMainActor {
            box.config = config
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_delegate_content_view")
public func vk_live_text_interaction_delegate_content_view(
    _ token: UnsafeMutableRawPointer?,
    _ outContentViewToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteractionDelegate requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionDelegateBox(token)
        outContentViewToken.pointee = try vkOnMainActor {
            box.contentViewBox.map(vkRetain)
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_delegate_set_content_view")
public func vk_live_text_interaction_delegate_set_content_view(
    _ token: UnsafeMutableRawPointer?,
    _ contentViewToken: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteractionDelegate requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionDelegateBox(token)
        let contentViewBox = try contentViewToken.map(vkLiveTextContentViewBox)
        try vkOnMainActor {
            box.contentViewBox = contentViewBox
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_delegate_recorded_events_json")
public func vk_live_text_interaction_delegate_recorded_events_json(
    _ token: UnsafeMutableRawPointer?,
    _ outEventsJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteractionDelegate requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionDelegateBox(token)
        try vkWriteJSON(try vkOnMainActor { box.recordedEvents }, to: outEventsJson)
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_delegate_clear_recorded_events")
public func vk_live_text_interaction_delegate_clear_recorded_events(
    _ token: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteractionDelegate requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionDelegateBox(token)
        try vkOnMainActor {
            box.clearRecordedEvents()
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_subject_release")
public func vk_live_text_subject_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    vkRelease(token)
}

@_cdecl("vk_live_text_subject_bounds")
public func vk_live_text_subject_bounds(
    _ token: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>,
    _ outY: UnsafeMutablePointer<Double>,
    _ outWidth: UnsafeMutablePointer<Double>,
    _ outHeight: UnsafeMutablePointer<Double>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextSubject requires macOS 13+"
            )
        }
        let box = try vkLiveTextSubjectBox(token)
        let bounds = try vkOnMainActor { box.subject.bounds }
        outX.pointee = bounds.origin.x
        outY.pointee = bounds.origin.y
        outWidth.pointee = bounds.size.width
        outHeight.pointee = bounds.size.height
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_subject_png_data")
public func vk_live_text_subject_png_data(
    _ token: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outLen: UnsafeMutablePointer<UInt64>,
    _ outWidth: UnsafeMutablePointer<Double>,
    _ outHeight: UnsafeMutablePointer<Double>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextSubject requires macOS 13+"
            )
        }
        let box = try vkLiveTextSubjectBox(token)
        let image = try vk_block_on_async(timeoutSeconds: 10) {
            try await box.subject.image
        }
        try vkWritePNGImage(
            image,
            to: outBytes,
            length: outLen,
            width: outWidth,
            height: outHeight
        )
        return VK_OK
    } catch ImageAnalysisOverlayView.SubjectUnavailable.imageUnavailable {
        outErrorMessage?.pointee = vkCString("subject image is unavailable")
        return VK_SUBJECT_UNAVAILABLE
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_new")
public func vk_live_text_interaction_new() -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else {
        return nil
    }
    return try? vkOnMainActor {
        vkRetain(VKLiveTextInteractionBox())
    }
}

@_cdecl("vk_live_text_interaction_new_with_delegate")
public func vk_live_text_interaction_new_with_delegate(
    _ delegateToken: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else {
        return nil
    }
    return try? vkOnMainActor {
        vkRetain(
            VKLiveTextInteractionBox(
                delegateBox: try vkLiveTextInteractionDelegateBox(delegateToken)
            )
        )
    }
}

@_cdecl("vk_live_text_interaction_release")
public func vk_live_text_interaction_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    vkRelease(token)
}

@_cdecl("vk_live_text_interaction_set_analysis")
public func vk_live_text_interaction_set_analysis(
    _ token: UnsafeMutableRawPointer?,
    _ analysisToken: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let analysisBox = try vkImageAnalysisBox(analysisToken)
        try vkOnMainActor {
            box.overlayView.analysis = analysisBox.analysis
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_track_image_at_path")
public func vk_live_text_interaction_track_image_at_path(
    _ token: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let path = try vkRequireFilePath(path, field: "path")
        let image = try vkLoadNSImage(at: path)
        try vkOnMainActor {
            let trackingImageViewBox = box.ensureTrackingImageViewBox()
            let size = image.size
            guard size.width > 0, size.height > 0 else {
                throw VKBridgeError.invalidArgument(
                    "tracked image had zero size"
                )
            }
            trackingImageViewBox.imageView.image = image
            trackingImageViewBox.imageView.frame = CGRect(
                origin: .zero,
                size: size
            )
            box.overlayView.frame = CGRect(origin: .zero, size: size)
            box.attachTrackingImageView()
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_delegate")
public func vk_live_text_interaction_delegate(
    _ token: UnsafeMutableRawPointer?,
    _ outDelegateToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        outDelegateToken.pointee = try vkOnMainActor {
            box.delegateBox.map(vkRetain)
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_set_delegate")
public func vk_live_text_interaction_set_delegate(
    _ token: UnsafeMutableRawPointer?,
    _ delegateToken: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let delegateBox = try delegateToken.map(vkLiveTextInteractionDelegateBox)
        try vkOnMainActor {
            box.delegateBox = delegateBox
            box.overlayView.delegate = delegateBox
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_preferred_interaction_types")
public func vk_live_text_interaction_preferred_interaction_types(
    _ token: UnsafeMutableRawPointer?,
    _ outTypesRaw: UnsafeMutablePointer<UInt64>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        outTypesRaw.pointee = try vkOnMainActor {
            UInt64(box.overlayView.preferredInteractionTypes.rawValue)
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_set_preferred_interaction_types")
public func vk_live_text_interaction_set_preferred_interaction_types(
    _ token: UnsafeMutableRawPointer?,
    _ typesRaw: UInt64,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        try vkOnMainActor {
            box.overlayView.preferredInteractionTypes =
                vkLiveTextInteractionTypes(from: typesRaw)
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_active_interaction_types")
public func vk_live_text_interaction_active_interaction_types(
    _ token: UnsafeMutableRawPointer?,
    _ outTypesRaw: UnsafeMutablePointer<UInt64>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        outTypesRaw.pointee = try vkOnMainActor {
            UInt64(box.overlayView.activeInteractionTypes.rawValue)
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_selectable_items_highlighted")
public func vk_live_text_interaction_selectable_items_highlighted(
    _ token: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        outValue.pointee = try vkOnMainActor {
            box.overlayView.selectableItemsHighlighted ? 1 : 0
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_set_selectable_items_highlighted")
public func vk_live_text_interaction_set_selectable_items_highlighted(
    _ token: UnsafeMutableRawPointer?,
    _ value: Int32,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        try vkOnMainActor {
            box.overlayView.selectableItemsHighlighted = value != 0
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_tracking_image_view")
public func vk_live_text_interaction_tracking_image_view(
    _ token: UnsafeMutableRawPointer?,
    _ outTrackingImageViewToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        outTrackingImageViewToken.pointee = try vkOnMainActor {
            box.trackingImageViewBox.map(vkRetain)
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_set_tracking_image_view")
public func vk_live_text_interaction_set_tracking_image_view(
    _ token: UnsafeMutableRawPointer?,
    _ trackingImageViewToken: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let trackingImageViewBox = try trackingImageViewToken.map(vkLiveTextTrackingImageViewBox)
        try vkOnMainActor {
            box.trackingImageViewBox = trackingImageViewBox
            box.attachTrackingImageView()
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_has_active_text_selection")
public func vk_live_text_interaction_has_active_text_selection(
    _ token: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        outValue.pointee = try vkOnMainActor {
            box.overlayView.hasActiveTextSelection ? 1 : 0
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_reset_selection")
public func vk_live_text_interaction_reset_selection(
    _ token: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        try vkOnMainActor {
            box.overlayView.resetSelection()
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_text")
public func vk_live_text_interaction_text(
    _ token: UnsafeMutableRawPointer?,
    _ outText: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let text = try vkOnMainActor {
            if #available(macOS 14.0, *) {
                return box.overlayView.text
            }
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction.text requires macOS 14+"
            )
        }
        outText.pointee = vkCString(text)
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_selected_text")
public func vk_live_text_interaction_selected_text(
    _ token: UnsafeMutableRawPointer?,
    _ outText: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let text = try vkOnMainActor {
            if #available(macOS 14.0, *) {
                return box.overlayView.selectedText
            }
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction.selectedText requires macOS 14+"
            )
        }
        outText.pointee = vkCString(text)
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_selected_attributed_text_json")
public func vk_live_text_interaction_selected_attributed_text_json(
    _ token: UnsafeMutableRawPointer?,
    _ outTextJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let payload = try vkOnMainActor { () -> VKAttributedTextPayload in
            if #available(macOS 14.0, *) {
                return vkAttributedTextPayload(from: box.overlayView.selectedAttributedText)
            }
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction.selectedAttributedText requires macOS 14+"
            )
        }
        try vkWriteJSON(payload, to: outTextJson)
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_selected_ranges_json")
public func vk_live_text_interaction_selected_ranges_json(
    _ token: UnsafeMutableRawPointer?,
    _ outRangesJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let ranges = try vkOnMainActor { () -> [VKTextRangePayload] in
            if #available(macOS 14.0, *) {
                let text = box.overlayView.text
                return box.overlayView.selectedRanges.map {
                    vkTextRangePayload(from: $0, in: text)
                }
            }
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction.selectedRanges requires macOS 14+"
            )
        }
        try vkWriteJSON(ranges, to: outRangesJson)
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_set_selected_ranges_json")
public func vk_live_text_interaction_set_selected_ranges_json(
    _ token: UnsafeMutableRawPointer?,
    _ rangesJson: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let ranges = try vkDecodeJSON(rangesJson, as: [VKTextRangePayload].self)
        try vkOnMainActor {
            if #available(macOS 14.0, *) {
                let text = box.overlayView.text
                box.overlayView.selectedRanges = try ranges.map {
                    try vkStringRange(from: $0, in: text)
                }
                return
            }
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction.selectedRanges requires macOS 14+"
            )
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_contents_rect")
public func vk_live_text_interaction_contents_rect(
    _ token: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>,
    _ outY: UnsafeMutablePointer<Double>,
    _ outWidth: UnsafeMutablePointer<Double>,
    _ outHeight: UnsafeMutablePointer<Double>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let rect = try vkOnMainActor { box.overlayView.contentsRect }
        outX.pointee = rect.origin.x
        outY.pointee = rect.origin.y
        outWidth.pointee = rect.size.width
        outHeight.pointee = rect.size.height
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_set_contents_rect_needs_update")
public func vk_live_text_interaction_set_contents_rect_needs_update(
    _ token: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        try vkOnMainActor {
            box.overlayView.setContentsRectNeedsUpdate()
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

func vkQueryLiveTextPointBool(
    token: UnsafeMutableRawPointer?,
    x: Double,
    y: Double,
    outValue: UnsafeMutablePointer<Int32>,
    outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    query: @escaping @MainActor (ImageAnalysisOverlayView, CGPoint) -> Bool
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let point = CGPoint(x: x, y: y)
        outValue.pointee = try vkOnMainActor {
            query(box.overlayView, point) ? 1 : 0
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_has_interactive_item_at_point")
public func vk_live_text_interaction_has_interactive_item_at_point(
    _ token: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    vkQueryLiveTextPointBool(
        token: token,
        x: x,
        y: y,
        outValue: outValue,
        outErrorMessage: outErrorMessage,
        query: { view, point in view.hasInteractiveItem(at: point) }
    )
}

@_cdecl("vk_live_text_interaction_has_text_at_point")
public func vk_live_text_interaction_has_text_at_point(
    _ token: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    vkQueryLiveTextPointBool(
        token: token,
        x: x,
        y: y,
        outValue: outValue,
        outErrorMessage: outErrorMessage,
        query: { view, point in view.hasText(at: point) }
    )
}

@_cdecl("vk_live_text_interaction_has_data_detector_at_point")
public func vk_live_text_interaction_has_data_detector_at_point(
    _ token: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    vkQueryLiveTextPointBool(
        token: token,
        x: x,
        y: y,
        outValue: outValue,
        outErrorMessage: outErrorMessage,
        query: { view, point in view.hasDataDetector(at: point) }
    )
}

@_cdecl("vk_live_text_interaction_has_supplementary_interface_at_point")
public func vk_live_text_interaction_has_supplementary_interface_at_point(
    _ token: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    vkQueryLiveTextPointBool(
        token: token,
        x: x,
        y: y,
        outValue: outValue,
        outErrorMessage: outErrorMessage,
        query: { view, point in view.hasSupplementaryInterface(at: point) }
    )
}

@_cdecl("vk_live_text_interaction_analysis_has_text_at_point")
public func vk_live_text_interaction_analysis_has_text_at_point(
    _ token: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    vkQueryLiveTextPointBool(
        token: token,
        x: x,
        y: y,
        outValue: outValue,
        outErrorMessage: outErrorMessage,
        query: { view, point in view.analysisHasText(at: point) }
    )
}

@_cdecl("vk_live_text_interaction_live_text_button_visible")
public func vk_live_text_interaction_live_text_button_visible(
    _ token: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        outValue.pointee = try vkOnMainActor {
            box.overlayView.liveTextButtonVisible ? 1 : 0
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_is_supplementary_interface_hidden")
public func vk_live_text_interaction_is_supplementary_interface_hidden(
    _ token: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        outValue.pointee = try vkOnMainActor {
            box.overlayView.isSupplementaryInterfaceHidden ? 1 : 0
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_set_supplementary_interface_hidden")
public func vk_live_text_interaction_set_supplementary_interface_hidden(
    _ token: UnsafeMutableRawPointer?,
    _ hidden: Int32,
    _ animated: Int32,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        try vkOnMainActor {
            box.overlayView.setSupplementaryInterfaceHidden(
                hidden != 0,
                animated: animated != 0
            )
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_supplementary_interface_content_insets")
public func vk_live_text_interaction_supplementary_interface_content_insets(
    _ token: UnsafeMutableRawPointer?,
    _ outTop: UnsafeMutablePointer<Double>,
    _ outLeft: UnsafeMutablePointer<Double>,
    _ outBottom: UnsafeMutablePointer<Double>,
    _ outRight: UnsafeMutablePointer<Double>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let insets = try vkOnMainActor {
            box.overlayView.supplementaryInterfaceContentInsets
        }
        outTop.pointee = insets.top
        outLeft.pointee = insets.left
        outBottom.pointee = insets.bottom
        outRight.pointee = insets.right
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_set_supplementary_interface_content_insets")
public func vk_live_text_interaction_set_supplementary_interface_content_insets(
    _ token: UnsafeMutableRawPointer?,
    _ top: Double,
    _ left: Double,
    _ bottom: Double,
    _ right: Double,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        try vkOnMainActor {
            box.overlayView.supplementaryInterfaceContentInsets = NSEdgeInsets(
                top: top,
                left: left,
                bottom: bottom,
                right: right
            )
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_menu_tags_json")
public func vk_live_text_menu_tags_json(
    _ outMenuTagsJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 14.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction.MenuTag requires macOS 14+"
            )
        }
        try vkWriteJSON(
            VKMenuTagsPayload(
                copyImage: ImageAnalysisOverlayView.MenuTag.copyImage,
                shareImage: ImageAnalysisOverlayView.MenuTag.shareImage,
                copySubject: ImageAnalysisOverlayView.MenuTag.copySubject,
                shareSubject: ImageAnalysisOverlayView.MenuTag.shareSubject,
                lookupItem: ImageAnalysisOverlayView.MenuTag.lookupItem,
                recommendedAppItems: ImageAnalysisOverlayView.MenuTag.recommendedAppItems
            ),
            to: outMenuTagsJson
        )
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_supplementary_interface_font_json")
public func vk_live_text_interaction_supplementary_interface_font_json(
    _ token: UnsafeMutableRawPointer?,
    _ outFontJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        #if compiler(>=5.3) && $NonescapableTypes
        let payload = try vkOnMainActor {
            box.overlayView.supplementaryInterfaceFont.map(vkFontPayload(from:))
        }
        try vkWriteJSON(payload, to: outFontJson)
        return VK_OK
        #else
        throw VKBridgeError.unavailableOnThisMacOS(
            "LiveTextInteraction.supplementaryInterfaceFont requires compiler support for nonescapable AppKit types"
        )
        #endif
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_set_supplementary_interface_font_json")
public func vk_live_text_interaction_set_supplementary_interface_font_json(
    _ token: UnsafeMutableRawPointer?,
    _ fontJson: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let payload = try vkDecodeJSON(fontJson, as: VKFontPayload?.self)
        #if compiler(>=5.3) && $NonescapableTypes
        try vkOnMainActor {
            box.overlayView.supplementaryInterfaceFont = try payload.map(vkFont(from:))
        }
        return VK_OK
        #else
        throw VKBridgeError.unavailableOnThisMacOS(
            "LiveTextInteraction.supplementaryInterfaceFont requires compiler support for nonescapable AppKit types"
        )
        #endif
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_begin_subject_analysis_if_necessary")
public func vk_live_text_interaction_begin_subject_analysis_if_necessary(
    _ token: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        try vkOnMainActor {
            box.overlayView.beginSubjectAnalysisIfNecessary()
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_subjects_json")
public func vk_live_text_interaction_subjects_json(
    _ token: UnsafeMutableRawPointer?,
    _ outSubjectsJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let subjects = try vk_block_on_main_actor_async(timeoutSeconds: 10) {
            await box.overlayView.subjects
        }
        try vkWriteJSON(vkSubjectTokens(from: subjects), to: outSubjectsJson)
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_highlighted_subjects_json")
public func vk_live_text_interaction_highlighted_subjects_json(
    _ token: UnsafeMutableRawPointer?,
    _ outSubjectsJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let subjects = try vkOnMainActor {
            box.overlayView.highlightedSubjects
        }
        try vkWriteJSON(vkSubjectTokens(from: subjects), to: outSubjectsJson)
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_set_highlighted_subjects_json")
public func vk_live_text_interaction_set_highlighted_subjects_json(
    _ token: UnsafeMutableRawPointer?,
    _ subjectsJson: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let subjects = try vkSubjects(from: subjectsJson)
        try vkOnMainActor {
            box.overlayView.highlightedSubjects = subjects
        }
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_subject_at_json")
public func vk_live_text_interaction_subject_at_json(
    _ token: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ outSubjectJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let subject = try vk_block_on_main_actor_async(timeoutSeconds: 10) {
            await box.overlayView.subject(at: CGPoint(x: x, y: y))
        }
        let token = subject.map { subject in
            vkTokenUInt64(vkRetain(VKLiveTextSubjectBox(subject: subject)))
        }
        try vkWriteJSON(token, to: outSubjectJson)
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_live_text_interaction_image_for_subjects_png_data")
public func vk_live_text_interaction_image_for_subjects_png_data(
    _ token: UnsafeMutableRawPointer?,
    _ subjectsJson: UnsafePointer<CChar>?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outLen: UnsafeMutablePointer<UInt64>,
    _ outWidth: UnsafeMutablePointer<Double>,
    _ outHeight: UnsafeMutablePointer<Double>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "LiveTextInteraction requires macOS 13+"
            )
        }
        let box = try vkLiveTextInteractionBox(token)
        let subjects = try vkSubjects(from: subjectsJson)
        let image = try vk_block_on_main_actor_async(timeoutSeconds: 10) {
            try await box.overlayView.image(for: subjects)
        }
        try vkWritePNGImage(
            image,
            to: outBytes,
            length: outLen,
            width: outWidth,
            height: outHeight
        )
        return VK_OK
    } catch ImageAnalysisOverlayView.SubjectUnavailable.imageUnavailable {
        outErrorMessage?.pointee = vkCString("subject image is unavailable")
        return VK_SUBJECT_UNAVAILABLE
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}
