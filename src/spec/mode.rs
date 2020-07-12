use serde::{Deserialize, Serialize};

/// Options for how to crop the media if the new resolution has a different aspect ratio.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum ResizeMode {
    /// The image is scaled to the maximum possible size that fits within the new resolution.
    #[serde(rename = "contain")]
    Contain,

    /// Crop the image to match the aspect ratio and resolution. Doesn't stretch.
    #[serde(rename = "crop")]
    Crop,

    /// Stretches the media to the exact resolution.
    #[serde(rename = "stretch")]
    Stretch,
}

impl Default for ResizeMode {
    fn default() -> Self {
        ResizeMode::Crop
    }
}
