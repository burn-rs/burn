use core::marker::PhantomData;

use crate::{
    backend::{Backend, BackendBridge},
    ops::FloatTensor,
    quantization::QTensorPrimitive,
};

use super::{RouterTensor, RunnerChannel};

pub struct BackendRouter<R: RunnerChannel> {
    r: PhantomData<R>,
}

impl<R: RunnerChannel> core::fmt::Debug for BackendRouter<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("router"))
    }
}

impl<R: RunnerChannel> Clone for BackendRouter<R> {
    fn clone(&self) -> Self {
        Self { r: PhantomData }
    }
}

impl<R: RunnerChannel> Default for BackendRouter<R> {
    fn default() -> Self {
        Self { r: PhantomData }
    }
}

impl<R: RunnerChannel> QTensorPrimitive for RouterTensor<R> {
    fn scheme(&self) -> &crate::quantization::QuantizationScheme {
        todo!()
    }

    fn strategy(&self) -> crate::quantization::QuantizationStrategy {
        todo!()
    }
}

/// Handle precision conversion.
#[derive(Debug)]
pub struct PrecisionBridge {}

impl<R: RunnerChannel> BackendBridge<BackendRouter<R>> for PrecisionBridge {
    type Target = BackendRouter<R>;

    fn into_target(
        tensor: FloatTensor<BackendRouter<R>>,
        _device: Option<<BackendRouter<R> as Backend>::Device>,
    ) -> FloatTensor<Self::Target> {
        todo!()
        // TODO: smilar to fusion `cast` in burn-fusion/src/bridge.rs
    }

    fn from_target(
        tensor: FloatTensor<Self::Target>,
        _device: Option<<BackendRouter<R> as Backend>::Device>,
    ) -> FloatTensor<BackendRouter<R>> {
        todo!()
    }
}

impl<C: RunnerChannel> Backend for BackendRouter<C> {
    type Device = C::Device;

    type FullPrecisionBridge = PrecisionBridge;

    type FloatTensorPrimitive = RouterTensor<C>;

    // TODO: how to set elem types?
    type FloatElem = f32;

    type IntTensorPrimitive = RouterTensor<C>;

    type IntElem = i32;

    type BoolTensorPrimitive = RouterTensor<C>;

    type QuantizedTensorPrimitive = RouterTensor<C>;

    type QuantizedEncoding = u32;

    fn name() -> String {
        todo!()
    }

    fn seed(seed: u64) {
        todo!()
    }
}

// Example usage:
// type MyBackend = BackendRouter<DirectChannel<(Cuda, NdArray, Wgpu), ByteBridge<(Cuda, NdArray, Wgpu)>>>
// ByteBridge is the default bridge for moving data between backends
// For efficient data movement/transfer, you can implement your own struct
