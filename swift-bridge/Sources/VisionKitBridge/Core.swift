import Foundation
import ImageIO
import VisionKit

let VK_OK: Int32 = 0
let VK_INVALID_ARGUMENT: Int32 = -1
let VK_UNAVAILABLE_ON_THIS_MACOS: Int32 = -2
let VK_TIMED_OUT: Int32 = -3
let VK_ANALYZER_NOT_SUPPORTED: Int32 = -10
let VK_FRAMEWORK_ERROR: Int32 = -20
let VK_UNKNOWN: Int32 = -99

@_cdecl("vk_string_free")
public func vk_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    guard let string else { return }
    free(string)
}

@inline(__always)
func vkCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
func vkRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func vkBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as type: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
func vkRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

enum VKBridgeError: Error, CustomStringConvertible {
    case invalidArgument(String)
    case unavailableOnThisMacOS(String)
    case timedOut(String)
    case analyzerNotSupported(String)
    case unknown(String)

    var description: String {
        switch self {
        case let .invalidArgument(message),
            let .unavailableOnThisMacOS(message),
            let .timedOut(message),
            let .analyzerNotSupported(message),
            let .unknown(message):
            return message
        }
    }

    var statusCode: Int32 {
        switch self {
        case .invalidArgument:
            return VK_INVALID_ARGUMENT
        case .unavailableOnThisMacOS:
            return VK_UNAVAILABLE_ON_THIS_MACOS
        case .timedOut:
            return VK_TIMED_OUT
        case .analyzerNotSupported:
            return VK_ANALYZER_NOT_SUPPORTED
        case .unknown:
            return VK_UNKNOWN
        }
    }
}

func vkStatus(from error: Error) -> Int32 {
    if let bridgeError = error as? VKBridgeError {
        return bridgeError.statusCode
    }
    return VK_FRAMEWORK_ERROR
}

final class VKAsyncResultBox<T>: @unchecked Sendable {
    private let lock = NSLock()
    private var storedResult: Result<T, Error>?

    func set(_ result: Result<T, Error>) {
        lock.lock()
        storedResult = result
        lock.unlock()
    }

    func get() -> Result<T, Error>? {
        lock.lock()
        defer { lock.unlock() }
        return storedResult
    }
}

public func vk_block_on_async<T>(
    timeoutSeconds: TimeInterval = 60,
    pollIntervalSeconds: TimeInterval = 0.01,
    work: @escaping () async throws -> T
) throws -> T {
    let box = VKAsyncResultBox<T>()
    Task {
        do {
            box.set(.success(try await work()))
        } catch {
            box.set(.failure(error))
        }
    }

    let deadline = Date().addingTimeInterval(timeoutSeconds)
    while box.get() == nil && Date() < deadline {
        RunLoop.current.run(mode: .default, before: Date().addingTimeInterval(pollIntervalSeconds))
    }

    guard let result = box.get() else {
        throw VKBridgeError.timedOut(
            "VisionKit async call timed out after \(Int(timeoutSeconds)) seconds"
        )
    }
    return try result.get()
}

func vkEncodeJSON<T: Encodable>(_ value: T) throws -> String {
    let data = try JSONEncoder().encode(value)
    guard let string = String(data: data, encoding: .utf8) else {
        throw VKBridgeError.unknown("failed to encode JSON as UTF-8")
    }
    return string
}

func vkDecodeJSON<T: Decodable>(_ cString: UnsafePointer<CChar>?, as type: T.Type) throws -> T {
    guard let cString else {
        throw VKBridgeError.invalidArgument("missing JSON payload")
    }
    let data = Data(String(cString: cString).utf8)
    do {
        return try JSONDecoder().decode(T.self, from: data)
    } catch {
        throw VKBridgeError.invalidArgument("invalid JSON payload: \(error.localizedDescription)")
    }
}

func vkRequireString(_ cString: UnsafePointer<CChar>?, field: String) throws -> String {
    guard let cString else {
        throw VKBridgeError.invalidArgument("missing \(field)")
    }
    return String(cString: cString)
}

func vkImageOrientation(from raw: UInt32) throws -> CGImagePropertyOrientation {
    guard let orientation = CGImagePropertyOrientation(rawValue: raw) else {
        throw VKBridgeError.invalidArgument("invalid image orientation raw value: \(raw)")
    }
    return orientation
}
