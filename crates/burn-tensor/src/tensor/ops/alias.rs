use crate::backend::{Backend, BackendBridge};

// We provide some type aliases to improve the readability of using associated types without
// having to use the disambiguation syntax.

/// Device type used by the backend.
pub type Device<B> = <B as Backend>::Device;

/// Float element type used by backend.
pub type FloatElem<B> = <B as Backend>::FloatElem;
/// Integer element type used by backend.
pub type IntElem<B> = <B as Backend>::IntElem;
/// Byte element type used by backend.
pub type ByteElem<B> = <B as Backend>::ByteElem;
/// Full precision float element type used by the backend.
pub type FullPrecisionBackend<B> =
    <<B as Backend>::FullPrecisionBridge as BackendBridge<B>>::Target;

/// Float tensor primitive type used by the backend.
pub type FloatTensor<B> = <B as Backend>::FloatTensorPrimitive;
/// Integer tensor primitive type used by the backend.
pub type IntTensor<B> = <B as Backend>::IntTensorPrimitive;
/// Boolean tensor primitive type used by the backend.
pub type BoolTensor<B> = <B as Backend>::BoolTensorPrimitive;
/// Byte tensor primitive type used by the backend.
pub type ByteTensor<B> = <B as Backend>::ByteTensorPrimitive;
/// Quantized tensor primitive type used by the backend.
pub type QuantizedTensor<B> = <B as Backend>::QuantizedTensorPrimitive;
