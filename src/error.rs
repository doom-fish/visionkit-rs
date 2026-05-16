use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiveTextSubjectUnavailable {
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
pub enum VisionKitError {
    InvalidArgument(String),
    UnavailableOnThisMacOS(String),
    UnavailableOnThisPlatform(String),
    TimedOut(String),
    AnalyzerNotSupported(String),
    Framework(String),
    LiveTextSubjectUnavailable(LiveTextSubjectUnavailable),
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
