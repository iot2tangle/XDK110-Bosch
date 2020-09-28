//!
//! Channel Library
//!
#![deny(
    bad_style,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features
)]
#![cfg_attr(not(debug_assertions), deny(warnings))]

pub mod channels_lite;
pub use channels_lite as channels;

pub mod utils;
