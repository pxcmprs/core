use super::{encoder, transform, PipelineError, PipelineResult};
use crate::spec::{Query, ResizeMode};
use gif::SetParameter;
use image::{DynamicImage, GenericImageView, RgbaImage};
use std::time::{Duration, Instant};

/// Response from a handler.
pub struct HandleResponse {
    /// How long the handling took.
    pub execution_time: Duration,

    /// Returned bytes, encoded in the specified format.
    pub bytes: Vec<u8>,
}

type HandleResult = PipelineResult<HandleResponse>;

pub fn handle_image(bytes: Vec<u8>, query: Query) -> HandleResult {
    let start = Instant::now();
    let mut image = image::load_from_memory(&bytes)?;
    let (nwidth, nheight) = (
        query.width.unwrap_or_else(|| image.width()),
        query.height.unwrap_or_else(|| image.height()),
    );
    image = transform::resize_dynimage(&image, query.mode.unwrap_or_default(), nwidth, nheight);
    let result = encoder::encode_image(&image, query.encoding)?;

    Ok(HandleResponse {
        bytes: result,
        execution_time: start.elapsed(),
    })
}

pub fn handle_gif(bytes: Vec<u8>, query: Query) -> HandleResult {
    if query.encoding.is_video() {
        let start = Instant::now();
        let mut decoder = gif::Decoder::new(bytes.as_slice());
        decoder.set(gif::ColorOutput::RGBA);
        let mut decoder = decoder.read_info().unwrap();

        let (owidth, oheight) = (decoder.width() as u32, decoder.height() as u32);

        let (nwidth, nheight) = (
            query.width.unwrap_or_else(|| owidth),
            query.height.unwrap_or_else(|| oheight),
        );

        let resize_mode = query.mode.unwrap_or_default();

        let (nwidth, nheight): (u32, u32) = match resize_mode {
            ResizeMode::Contain => resize_dimensions(owidth, oheight, nwidth, nheight, false),
            ResizeMode::Crop => (nwidth, nheight),
            ResizeMode::Stretch => (nwidth, nheight),
        };

        let mut output: Vec<u8> = Vec::new();

        {
            let mut encoder =
                gif::Encoder::new(&mut output, nwidth as u16, nheight as u16, &[]).unwrap();
            encoder.set(gif::Repeat::Infinite).unwrap();

            while let Some(frame) = decoder.read_next_frame().unwrap() {
                let rgba = match RgbaImage::from_raw(owidth, oheight, frame.buffer.to_vec()) {
                    Some(buffer) => buffer,
                    None => return Err(PipelineError::Unknown),
                };
                let resized = transform::resize_dynimage(
                    &DynamicImage::ImageRgba8(rgba),
                    resize_mode,
                    nwidth,
                    nheight,
                );
                let mut new_rgba = resized.to_rgba().to_vec();
                let (width, height) = resized.dimensions();

                let frame =
                    gif::Frame::from_rgba_speed(width as u16, height as u16, &mut *new_rgba, 30);

                encoder.write_frame(&frame).unwrap();
            }
        }

        Ok(HandleResponse {
            bytes: output,
            execution_time: start.elapsed(),
        })
    } else {
        handle_image(bytes, query)
    }
}

/// Calculates the width and height an image should be resized to.
/// This preserves aspect ratio, and based on the `fill` parameter
/// will either fill the dimensions to fit inside the smaller constraint
/// (will overflow the specified bounds on one axis to preserve
/// aspect ratio), or will shrink so that both dimensions are
/// completely contained with in the given `width` and `height`,
/// with empty space on one axis.
fn resize_dimensions(width: u32, height: u32, nwidth: u32, nheight: u32, fill: bool) -> (u32, u32) {
    let ratio = u64::from(width) * u64::from(nheight);
    let nratio = u64::from(nwidth) * u64::from(height);

    let use_width = if fill {
        nratio > ratio
    } else {
        nratio <= ratio
    };
    let intermediate = if use_width {
        u64::from(height) * u64::from(nwidth) / u64::from(width)
    } else {
        u64::from(width) * u64::from(nheight) / u64::from(height)
    };
    if use_width {
        if intermediate <= u64::from(::std::u32::MAX) {
            (nwidth, intermediate as u32)
        } else {
            (
                (u64::from(nwidth) * u64::from(::std::u32::MAX) / intermediate) as u32,
                ::std::u32::MAX,
            )
        }
    } else if intermediate <= u64::from(::std::u32::MAX) {
        (intermediate as u32, nheight)
    } else {
        (
            ::std::u32::MAX,
            (u64::from(nheight) * u64::from(::std::u32::MAX) / intermediate) as u32,
        )
    }
}
