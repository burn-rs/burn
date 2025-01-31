use crate::VisionOps;

#[cfg(feature = "autodiff")]
use burn_autodiff::{checkpoint::strategy::CheckpointStrategy, Autodiff};
#[cfg(feature = "candle")]
use burn_candle::{Candle, FloatCandleElement, IntCandleElement};
#[cfg(feature = "ndarray")]
use burn_ndarray::{FloatNdArrayElement, IntNdArrayElement, NdArray, QuantElement};
#[cfg(feature = "tch")]
use burn_tch::{LibTorch, TchElement};

#[cfg(feature = "ndarray")]
impl<E: FloatNdArrayElement, I: IntNdArrayElement, Q: QuantElement> VisionOps<Self>
    for NdArray<E, I, Q>
{
}

#[cfg(feature = "candle")]
impl<F: FloatCandleElement, I: IntCandleElement> VisionOps<Self> for Candle<F, I> {}
#[cfg(feature = "tch")]
impl<E: TchElement, Q: burn_tch::QuantElement> VisionOps<Self> for LibTorch<E, Q> {}
#[cfg(feature = "autodiff")]
impl<B: burn_tensor::backend::Backend, C: CheckpointStrategy> VisionOps<Self> for Autodiff<B, C> {}
