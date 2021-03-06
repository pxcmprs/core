/// Encoding of media.
pub mod encoder;

/// Errors.
pub mod error;
mod handler;
mod transform;

use crate::spec::Query;
use error::PipelineError;
use image::ImageFormat;
use mime::Mime;
use std::time::Duration;

type PipelineResult<T> = Result<T, PipelineError>;

/// Response from a pipeline.
pub struct PipelineResponse {
    /// A byte vector containing the encoded media.
    pub bytes: Vec<u8>,

    /// The mime type of the encoded media.
    pub mime: Mime,

    /// Execution time.
    pub execution_time: Duration
}

/// Handle a `Query`.
pub fn handle_query(bytes: Vec<u8>, query: Query) -> PipelineResult<PipelineResponse> {
    let format = image::guess_format(&bytes)?;
    let mime = query.encoding.mime_type();

    let response: handler::HandleResponse = match format {
        ImageFormat::Gif => handler::handle_gif(bytes, query)?,
        _ => handler::handle_image(bytes, query)?,
    };

    Ok(PipelineResponse {
        bytes: response.bytes,
        mime,
        execution_time: response.execution_time
    })
}
