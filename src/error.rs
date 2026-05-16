use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisionKitError {
    InvalidArgument(String),
    UnavailableOnThisMacOS(String),
    UnavailableOnThisPlatform(String),
    TimedOut(String),
    AnalyzerNotSupported(String),
    Framework(String),
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
        }
    }
}

impl Error for VisionKitError {}
