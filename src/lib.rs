#![deny(missing_docs)]

//! # `pxcmprs-core`
//! This library contains the fundamental components of pxcmprs.
//! If you only need the basic structs for the API, disable all default features like so:
//! ```toml
//! [dependencies.pxcmprs-core]
//! default-features = false
//! ```

///
pub mod spec;

#[cfg(feature = "pipeline")]
/// Media processing pipeline. Requires the `pipeline` feature.
pub mod pipeline;
