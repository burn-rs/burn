mod base;
mod padding;

/// Loading to shared memory is done in a contiguous manner
pub mod contiguous;
/// Loading is done in a contiguous manner, with left hand tensor being transposed.
pub mod contiguous_vectorized;
/// Loading is done in a tile manner
pub mod tile;
/// Loading is done in a tile manner, with left hand tensor being transposed.
pub mod tile_vectorized;
/// WGSL vec4 primitives are used on left hand tensor
pub mod vec4_primitive;
/// WGSL vec4 primitives are used on left and right hand tensor
pub mod vec4_rhs;

pub use contiguous_vectorized::*;
