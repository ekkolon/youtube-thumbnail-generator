use clap::ValueEnum;
use image::imageops::FilterType;

/// Sampling algorithm to use for thumbnail generation.
///
/// NOTE:
/// This is equivalent to the `FilterType` provided by the *image* crate.
///
/// We duplicate this enum to make it compatible with *clap*s `ValueEnum`.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq)]
pub enum SamplingFilter {
    /// Nearest Neighbor
    Nearest,

    /// Linear Filter
    Triangle,

    /// Cubic Filter
    CatmullRom,

    /// Gaussian Filter
    Gaussian,

    /// Lanczos with window 3
    Lanczos3,
}

impl SamplingFilter {
    /// Convert a sampling filter back to native `image::FilterType`.
    pub fn to_filter_type(self) -> FilterType {
        match &self {
            SamplingFilter::Nearest => FilterType::Nearest,
            SamplingFilter::Triangle => FilterType::Triangle,
            SamplingFilter::CatmullRom => FilterType::CatmullRom,
            SamplingFilter::Gaussian => FilterType::Gaussian,
            SamplingFilter::Lanczos3 => FilterType::Lanczos3,
        }
    }
}
