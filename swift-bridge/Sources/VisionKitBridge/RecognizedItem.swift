import Foundation

@_cdecl("vk_recognized_item_support_json")
public func vk_recognized_item_support_json(
    _ outSupportJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        try vkWriteJSON(
            vkAreaSupportPayload(
                area: "RecognizedItem",
                availableOnCurrentPlatform: false,
                availability: "iOS 16+; @available(macCatalyst, unavailable)",
                reason: "RecognizedItem is tied to DataScannerViewController's live camera scanning pipeline and does not exist in macOS VisionKit.",
                members: [
                    "Bounds.{topLeft,topRight,bottomRight,bottomLeft}",
                    "case text(RecognizedItem.Text)",
                    "case barcode(RecognizedItem.Barcode)",
                    "id",
                    "bounds",
                ],
                notes: [
                    "This crate surfaces the iOS-only item family as availability metadata on macOS.",
                ]
            ),
            to: outSupportJson
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
