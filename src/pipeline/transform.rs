use crate::spec::ResizeMode;
use image::DynamicImage;

/// Resize a `DynamicImage`.
pub fn resize_dynimage(
    image: &DynamicImage,
    mode: &ResizeMode,
    width: u32,
    height: u32,
) -> DynamicImage {
    match mode {
        ResizeMode::Contain => image.thumbnail(width, height),
        ResizeMode::Crop => {
            image.resize_to_fill(width, height, image::imageops::FilterType::Triangle)
        }
        ResizeMode::Stretch => image.thumbnail_exact(width, height),
    }
}
