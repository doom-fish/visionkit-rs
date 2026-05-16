import Foundation

@_cdecl("vk_barcode_support_json")
public func vk_barcode_support_json(
    _ outSupportJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        try vkWriteJSON(
            vkAreaSupportPayload(
                area: "Barcode",
                availableOnCurrentPlatform: false,
                availability: "iOS 16+; part of RecognizedItem.Barcode; @available(macCatalyst, unavailable)",
                reason: "RecognizedItem.Barcode is nested under DataScannerViewController's iOS-only live camera APIs and is not present in macOS VisionKit.",
                members: [
                    "id",
                    "bounds",
                    "payloadStringValue",
                    "observation",
                ],
                notes: [
                    "Use ImageAnalysisTypes::MACHINE_READABLE_CODE on macOS to request barcode detection from still images.",
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
