use image::ImageError;
use std::{error, fmt};

/// Errors related to encoding.
#[derive(Debug)]
pub enum EncodeError {
    /// The media cannot be encoded as it is unsupported.
    Unsupported,

    /// Error related to the `image` crate.
    ImageError(ImageError),
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncodeError::Unsupported => {
                write!(f, "The media cannot be encoded as it is not supported.")
            }
            EncodeError::ImageError(e) => e.fmt(f),
        }
    }
}

impl error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            EncodeError::Unsupported => None,
            EncodeError::ImageError(ref e) => Some(e),
        }
    }
}

impl From<ImageError> for EncodeError {
    fn from(err: ImageError) -> EncodeError {
        EncodeError::ImageError(err)
    }
}

/// Error thrown by the pipeline.
#[derive(Debug)]
pub enum PipelineError {
    /// Unknown error.
    Unknown,

    /// Image error.
    ImageError(ImageError),

    /// Encode error.
    EncodeError(EncodeError),
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PipelineError::Unknown => write!(f, "An unknown error occurred."),
            PipelineError::ImageError(e) => e.fmt(f),
            PipelineError::EncodeError(e) => e.fmt(f),
        }
    }
}

impl error::Error for PipelineError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            PipelineError::Unknown => None,
            PipelineError::ImageError(ref e) => Some(e),
            PipelineError::EncodeError(ref e) => Some(e),
        }
    }
}

impl From<ImageError> for PipelineError {
    fn from(err: ImageError) -> PipelineError {
        PipelineError::ImageError(err)
    }
}

impl From<EncodeError> for PipelineError {
    fn from(err: EncodeError) -> PipelineError {
        PipelineError::EncodeError(err)
    }
}
