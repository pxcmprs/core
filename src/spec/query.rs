use super::{Encoding, ResizeMode};
use serde::{Deserialize, Serialize};

/// The default struct for requests to pxcmprs.
#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    /// URL of the source image. Input formats currently supported are the same as those of the [image] crate.
    pub source: String,

    /// The width of the processed image.
    pub width: Option<u32>,

    /// The height of the processed image.
    pub height: Option<u32>,

    /// Output format of the image.
    #[serde(alias = "format")]
    pub encoding: Encoding,

    /// Whether or not to crop the media if the new aspect ratio doesn't match.
    pub mode: Option<ResizeMode>,
}
