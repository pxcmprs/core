use super::error::EncodeError;
use crate::spec::Encoding;
use image::{DynamicImage, GenericImageView, ImageOutputFormat};

/// Encoding result.
pub type EncodeResult = Result<Vec<u8>, EncodeError>;

fn encode_default_image(image: &DynamicImage, format: ImageOutputFormat) -> EncodeResult {
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut bytes, format)?;
    Ok(bytes)
}

/// Encode a `DynamicImage` in a specified `Encoding`.
pub fn encode_image(image: &DynamicImage, encoding: &Encoding) -> EncodeResult {
    use Encoding::*;

    match encoding {
        WebP => {
            let (width, height) = image.dimensions();
            let encoder: webp::Encoder = match image {
                DynamicImage::ImageRgb8(image) => {
                    webp::Encoder::from_rgb(image.as_ref(), width, height)
                }
                DynamicImage::ImageRgba8(image) => {
                    webp::Encoder::from_rgba(image.as_ref(), width, height)
                }
                _ => return Err(EncodeError::Unsupported),
            };
            Ok(encoder.encode(85.0).to_vec())
        }
        Jpeg => encode_default_image(image, ImageOutputFormat::Jpeg(85)),
        Png => encode_default_image(image, ImageOutputFormat::Png),
        Gif => encode_default_image(image, ImageOutputFormat::Gif),
    }
}
