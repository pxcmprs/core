use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Output format of the processed image.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Encoding {
    /// Output the media as JPEG.
    #[serde(rename = "jpeg")]
    #[serde(alias = "jpg")]
    Jpeg,

    /// Output the media as a PNG file.
    #[serde(rename = "png")]
    Png,

    /// Encode the media with WebP.
    #[serde(rename = "webp")]
    WebP,

    /// Encode the media as a GIF.
    #[serde(rename = "gif")]
    Gif,
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::Jpeg
    }
}

impl Encoding {
    /// Get the `Mime` type.
    pub fn mime_type(self) -> Mime {
        match self {
            Encoding::Jpeg => IMAGE_JPEG,
            Encoding::Png => IMAGE_PNG,
            Encoding::WebP => Mime::from_str("image/webp").unwrap(),
            Encoding::Gif => IMAGE_GIF,
        }
    }

    /// Returns `true` if the encoding is considered a video, for example a MP4 or GIF (even though GIFs are not neccessarily animated).
    pub fn is_video(self) -> bool {
        match self {
            Encoding::Gif => true,
            _ => false,
        }
    }
}
