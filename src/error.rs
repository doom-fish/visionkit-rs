use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents why a VisionKit live text subject is unavailable.
pub enum LiveTextSubjectUnavailable {
    /// Represents a VisionKit subject image that could not be produced.
    ImageUnavailable,
}

impl fmt::Display for LiveTextSubjectUnavailable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageUnavailable => f.write_str("subject image is unavailable"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents an error returned by the VisionKit wrappers.
pub enum VisionKitError {
    /// Represents an invalid-argument error from VisionKit.
    InvalidArgument(String),
    /// Represents a macOS availability error from VisionKit.
    UnavailableOnThisMacOS(String),
    /// Represents a platform availability error from VisionKit.
    UnavailableOnThisPlatform(String),
    /// Represents a timeout reported by VisionKit.
    TimedOut(String),
    /// Represents an unsupported-analyzer error from VisionKit.
    AnalyzerNotSupported(String),
    /// Represents a framework-level error from VisionKit.
    Framework(String),
    /// Represents a live text subject error from VisionKit.
    LiveTextSubjectUnavailable(LiveTextSubjectUnavailable),
    /// Represents an unknown error returned by VisionKit.
    Unknown(String),
}

impl fmt::Display for VisionKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message)
            | Self::UnavailableOnThisMacOS(message)
            | Self::UnavailableOnThisPlatform(message)
            | Self::TimedOut(message)
            | Self::AnalyzerNotSupported(message)
            | Self::Framework(message)
            | Self::Unknown(message) => f.write_str(message),
            Self::LiveTextSubjectUnavailable(kind) => kind.fmt(f),
        }
    }
}

impl Error for VisionKitError {}
