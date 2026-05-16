import AppKit
import Foundation
import VisionKit

@available(macOS 13.0, *)
@MainActor
final class VKLiveTextInteractionBox: NSObject {
    let overlayView: ImageAnalysisOverlayView
    let trackingImageView: NSImageView

    override init() {
        overlayView = ImageAnalysisOverlayView(frame: .zero)
        trackingImageView = NSImageView(frame: .zero)
        trackingImageView.imageScaling = .scaleProportionallyUpOrDown
        super.init()
        #if compiler(>=5.3) && $NonescapableTypes
        overlayView.trackingImageView = trackingImageView
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

@_cdecl("vk_live_text_interaction_new")
public func vk_live_text_interaction_new() -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else {
        return nil
    }
    return try? vkOnMainActor {
        vkRetain(VKLiveTextInteractionBox())
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
            let size = image.size
            guard size.width > 0, size.height > 0 else {
                throw VKBridgeError.invalidArgument(
                    "tracked image had zero size"
                )
            }
            box.trackingImageView.image = image
            box.trackingImageView.frame = CGRect(
                origin: .zero,
                size: size
            )
            box.overlayView.frame = CGRect(origin: .zero, size: size)
            #if compiler(>=5.3) && $NonescapableTypes
            box.overlayView.trackingImageView = box.trackingImageView
            #endif
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
