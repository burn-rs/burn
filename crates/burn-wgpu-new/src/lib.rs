#[macro_use]
extern crate derive_new;

extern crate alloc;

mod compiler;
mod compute;
mod device;
mod element;
mod fusion;
mod graphics;
mod runtime;

use burn_wgpu::JitBackend;
pub use device::*;
pub use element::*;
pub use graphics::*;
use runtime::WgpuRuntime;

#[cfg(feature = "fusion")]
/// Tensor backend that uses the [wgpu] crate for executing GPU compute shaders.
///
/// This backend can target multiple graphics APIs, including:
///   - [Vulkan] on Linux, Windows, and Android.
///   - [OpenGL](crate::OpenGl) on Linux, Windows, and Android.
///   - [DirectX 12](crate::Dx12) on Windows.
///   - [Metal] on Apple hardware.
///   - [WebGPU](crate::WebGpu) on supported browsers and `wasm` runtimes.
///
/// # Notes
///
/// This version of the [wgpu] backend uses [burn_fusion] to compile and optimize streams of tensor
/// operations for improved performance.
///
/// You can disable the `fusion` feature flag to remove that functionality, which might be
/// necessary on `wasm` for now.
pub type Wgpu<G = AutoGraphicsApi, F = f32, I = i32> =
    burn_fusion::Fusion<JitBackend<WgpuRuntime<G, F, I>>>;

#[cfg(not(feature = "fusion"))]
/// Tensor backend that uses the [wgpu] crate for executing GPU compute shaders.
///
/// This backend can target multiple graphics APIs, including:
///   - [Vulkan] on Linux, Windows, and Android.
///   - [OpenGL](crate::OpenGl) on Linux, Windows, and Android.
///   - [DirectX 12](crate::Dx12) on Windows.
///   - [Metal] on Apple hardware.
///   - [WebGPU](crate::WebGpu) on supported browsers and `wasm` runtimes.
///
/// # Notes
///
/// This version of the [wgpu] backend doesn't use [burn_fusion] to compile and optimize streams of tensor
/// operations.
///
/// You can enable the `fusion` feature flag to add that functionality, which might improve
/// performance.
pub type Wgpu<G = AutoGraphicsApi, F = f32, I = i32> = JitBackend<WgpuRuntime<G, F, I>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WgpuRuntime;

    pub type TestCompiler = crate::compiler::wgsl::Compiler<f32, i32>;
    pub type TestRuntime = WgpuRuntime<AutoGraphicsApi, f32, i32>;
    pub type TestBackend = JitBackend<TestRuntime>;

    pub type TestTensor<const D: usize> = burn_tensor::Tensor<TestBackend, D>;
    pub type TestTensorInt<const D: usize> = burn_tensor::Tensor<TestBackend, D, burn_tensor::Int>;
    pub type TestTensorBool<const D: usize> =
        burn_tensor::Tensor<TestBackend, D, burn_tensor::Bool>;

    burn_tensor::testgen_all!();
    burn_autodiff::testgen_all!();
}
