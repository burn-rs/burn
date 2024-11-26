use cubecl::linalg::matmul::components::global;

pub trait Config: global::Config {
    fn out_shape(&self, dim: u32) -> u32;
    fn kernel_size(&self, dim: u32) -> u32;
    fn dilation(&self, dim: u32) -> u32;
    fn stride(&self, dim: u32) -> u32;
    fn padding(&self, dim: u32) -> i32;
}
