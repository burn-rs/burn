mod interpolate1d;
mod interpolate2d;

pub use interpolate1d::*;
pub use interpolate2d::*;

use crate::tensor::ops::InterpolateMode as OpsInterpolateMode;

/// Coordinate transformation mode using scale_factor
///
/// This enum defines different modes for coordinate transformation using a scale factor.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum CoordinateTransformationMode {
    /// Transforms coordinates using the formula: x_resized = (x_original + 0.5) * scale - 0.5
    HalfPixel,

    /// Transforms coordinates using the formula: x_resized = x_original * scale
    Asymmetric,
}

/// Algorithm used for downsampling and upsampling
///
/// This enum defines different interpolation modes for resampling data.
#[derive(new, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum InterpolateMode {
    /// Nearest-neighbor interpolation
    ///
    /// This mode selects the value of the nearest sample point for each output pixel.
    /// It is applicable for both temporal and spatial data.
    Nearest,

    /// Linear interpolation
    ///
    /// This mode calculates the output value using linear
    /// interpolation between nearby sample points.
    ///
    /// It is applicable for both temporal and spatial data.
    Linear,

    /// Cubic interpolation
    ///
    /// This mode uses cubic interpolation to calculate the output value
    /// based on surrounding sample points.
    ///
    /// It is applicable for both temporal and spatial data and generally
    /// provides smoother results than linear interpolation.
    Cubic,
}

impl From<InterpolateMode> for OpsInterpolateMode {
    fn from(mode: InterpolateMode) -> Self {
        match mode {
            InterpolateMode::Nearest => OpsInterpolateMode::Nearest,
            InterpolateMode::Linear => OpsInterpolateMode::Bilinear,
            InterpolateMode::Cubic => OpsInterpolateMode::Bicubic,
        }
    }
}
