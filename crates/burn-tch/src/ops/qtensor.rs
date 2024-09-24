use std::ops::Range;

use burn_tensor::{
    ops::{FloatTensor, IntTensor, QTensorOps, QuantizedTensor},
    quantization::{
        QTensorPrimitive, Quantization, QuantizationParametersPrimitive, QuantizationScheme,
        QuantizationStrategy, QuantizationType,
    },
    DType, Shape, TensorData,
};

use crate::{LibTorch, LibTorchDevice, QuantElement, TchElement, TchQTensor, TchShape, TchTensor};

use super::TchOps;

impl<E: TchElement, Q: QuantElement> QTensorOps<Self> for LibTorch<E, Q> {
    fn q_from_data(data: TensorData, device: &LibTorchDevice) -> QuantizedTensor<Self> {
        let shape_tch = TchShape::from(data.shape.as_slice());
        let device = (*device).into();

        // NOTE: tch-rs doesn't have `from_blob_quantized_*` APIs
        // https://github.com/pytorch/pytorch/blob/main/aten/src/ATen/quantized/Quantizer.cpp#L322
        // So for now we have to load the dequantized values to quantize them back since the dequantization
        // methods take the values provided when quantizing.
        let (tensor, scheme) = match data.dtype {
            DType::QFloat(strategy) => match strategy {
                QuantizationStrategy::PerTensorAffineInt8(q) => {
                    let values = q.dequantize(&data.iter::<i8>().collect::<Vec<_>>());
                    let tensor = tch::Tensor::from_slice(&values).to(device);
                    let tensor = TchOps::<E>::quantize::<i8>(
                        TchTensor::new(tensor.reshape(shape_tch.dims)),
                        &strategy,
                    )
                    .tensor;
                    (tensor, strategy.scheme())
                }
                QuantizationStrategy::PerTensorSymmetricInt8(q) => {
                    let values = q.dequantize(&data.iter::<i8>().collect::<Vec<_>>());
                    let tensor = tch::Tensor::from_slice(&values).to(device);
                    let tensor = TchOps::<E>::quantize::<i8>(
                        TchTensor::new(tensor.reshape(shape_tch.dims)),
                        &strategy,
                    )
                    .tensor;
                    (tensor, strategy.scheme())
                }
            },
            _ => panic!(
                "Invalid dtype (expected DType::QFloat, got {:?})",
                data.dtype
            ),
        };
        TchQTensor {
            qtensor: TchTensor::new(tensor),
            scheme,
        }
    }

    fn quantize(
        tensor: FloatTensor<Self>,
        scheme: &QuantizationScheme,
        qparams: QuantizationParametersPrimitive<Self>,
    ) -> QuantizedTensor<Self> {
        let mut tensor = tensor;
        // Quantize only works on Float Tensor
        if E::dtype() == DType::F16 {
            tensor.tensor = tensor.tensor.to_kind(tch::Kind::Float);
        }

        let qtensor = match scheme {
            QuantizationScheme::PerTensorAffine(dtype) => match dtype {
                QuantizationType::QInt8 => tensor.tensor.quantize_per_tensor_tensor_qparams(
                    &qparams.scale.tensor,
                    &qparams.offset.unwrap().tensor,
                    tch::Kind::QInt8,
                ),
            },
            QuantizationScheme::PerTensorSymmetric(_) => {
                tensor.tensor.quantize_per_tensor_tensor_qparams(
                    &qparams.scale.tensor,
                    &tch::Tensor::zeros_like(&qparams.scale.tensor),
                    tch::Kind::QInt8,
                )
            }
        };

        TchQTensor {
            qtensor: TchTensor::new(qtensor),
            scheme: scheme.clone(),
        }
    }

    fn quantize_dynamic(
        tensor: FloatTensor<Self>,
        scheme: &QuantizationScheme,
    ) -> QuantizedTensor<Self> {
        let qtensor = match &scheme {
            QuantizationScheme::PerTensorAffine(dtype) => match dtype {
                // Notes on `reduce_range`:
                // https://github.com/pytorch/pytorch/issues/93140
                // https://onnxruntime.ai/docs/performance/model-optimizations/quantization.html#data-type-selection
                QuantizationType::QInt8 => tensor
                    .tensor
                    .quantize_per_tensor_dynamic(tch::Kind::QInt8, /*reduce_range*/ false),
            },
            QuantizationScheme::PerTensorSymmetric(dtype) => {
                log::warn!("LibTorch backend does not support symmetric per-tensor scheme for dynamic quantization, reverting to the default per-tensor affine quantization");
                match dtype {
                    QuantizationType::QInt8 => tensor
                        .tensor
                        .quantize_per_tensor_dynamic(tch::Kind::QInt8, /*reduce_range*/ false),
                }
            }
        };

        TchQTensor {
            qtensor: TchTensor::new(qtensor),
            scheme: scheme.clone(),
        }
    }

    fn dequantize(tensor: QuantizedTensor<Self>) -> FloatTensor<Self> {
        TchTensor::new(tensor.qtensor.tensor.dequantize().to_kind(E::KIND))
    }

    fn q_shape(tensor: &QuantizedTensor<Self>) -> Shape {
        tensor.qtensor.shape()
    }

    fn q_device(tensor: &QuantizedTensor<Self>) -> LibTorchDevice {
        tensor.qtensor.tensor.device().into()
    }

    fn q_to_device(
        tensor: QuantizedTensor<Self>,
        device: &burn_tensor::Device<Self>,
    ) -> QuantizedTensor<Self> {
        let mut tensor = tensor;
        tensor.qtensor = TchOps::to_device(tensor.qtensor, device);
        tensor
    }

    fn q_reshape(tensor: QuantizedTensor<Self>, shape: Shape) -> QuantizedTensor<Self> {
        TchQTensor {
            qtensor: TchOps::reshape(tensor.qtensor, shape),
            scheme: tensor.scheme,
        }
    }

    async fn q_into_data(tensor: QuantizedTensor<Self>) -> TensorData {
        let shape = Self::q_shape(&tensor);
        let tensor = Self::q_reshape(tensor.clone(), Shape::new([shape.num_elements()]));
        let strategy = tensor.strategy();

        // To get the integer values we have to call `int_repr()`
        let values: Result<Vec<i8>, tch::TchError> = tensor.qtensor.tensor.int_repr().try_into();

        TensorData::quantized(values.unwrap(), shape, strategy)
    }

    fn q_swap_dims(
        tensor: QuantizedTensor<Self>,
        dim1: usize,
        dim2: usize,
    ) -> QuantizedTensor<Self> {
        // NOTE: with per-channel quantization (future), the channel axis could be impacted by this op
        let mut tensor = tensor;
        tensor.qtensor = TchOps::swap_dims(tensor.qtensor, dim1, dim2);
        tensor
    }

    fn q_permute(tensor: QuantizedTensor<Self>, axes: &[usize]) -> QuantizedTensor<Self> {
        // NOTE: with per-channel quantization (future), the channel axis could be impacted by this op
        let mut tensor = tensor;
        tensor.qtensor = TchOps::permute(tensor.qtensor, axes);
        tensor
    }

    fn q_flip(tensor: QuantizedTensor<Self>, axes: &[usize]) -> QuantizedTensor<Self> {
        let mut tensor = tensor;
        tensor.qtensor = TchOps::flip(tensor.qtensor, axes);
        tensor
    }

    fn q_select(
        tensor: QuantizedTensor<Self>,
        dim: usize,
        indices: IntTensor<Self>,
    ) -> QuantizedTensor<Self> {
        let mut tensor = tensor;
        tensor.qtensor = TchOps::index_select_dim(tensor.qtensor, dim, indices);
        tensor
    }

    fn q_slice(tensor: QuantizedTensor<Self>, ranges: &[Range<usize>]) -> QuantizedTensor<Self> {
        let mut tensor = tensor;
        tensor.qtensor = TchOps::slice(tensor.qtensor, ranges);
        tensor
    }

    fn q_argmax(tensor: QuantizedTensor<Self>, dim: usize) -> IntTensor<Self> {
        TchOps::argmax(TchTensor::<Q>::new(tensor.qtensor.tensor.int_repr()), dim)
    }

    fn q_argmin(tensor: QuantizedTensor<Self>, dim: usize) -> IntTensor<Self> {
        TchOps::argmin(TchTensor::<Q>::new(tensor.qtensor.tensor.int_repr()), dim)
    }

    fn q_max_dim_with_indices(
        tensor: QuantizedTensor<Self>,
        dim: usize,
    ) -> (QuantizedTensor<Self>, IntTensor<Self>) {
        let (qtensor, indices) = TchOps::max_dim_with_indices(tensor.qtensor, dim);
        let values = TchQTensor {
            qtensor,
            scheme: tensor.scheme,
        };
        (values, indices)
    }

    fn q_max_dim(tensor: QuantizedTensor<Self>, dim: usize) -> QuantizedTensor<Self> {
        TchQTensor {
            qtensor: TchOps::max_dim(tensor.qtensor, dim),
            scheme: tensor.scheme,
        }
    }

    fn q_min_dim(tensor: QuantizedTensor<Self>, dim: usize) -> QuantizedTensor<Self> {
        TchQTensor {
            qtensor: TchOps::min_dim(tensor.qtensor, dim),
            scheme: tensor.scheme,
        }
    }

    fn q_min_dim_with_indices(
        tensor: QuantizedTensor<Self>,
        dim: usize,
    ) -> (QuantizedTensor<Self>, IntTensor<Self>) {
        let (qtensor, indices) = TchOps::min_dim_with_indices(tensor.qtensor, dim);
        let values = TchQTensor {
            qtensor,
            scheme: tensor.scheme,
        };
        (values, indices)
    }

    fn q_narrow(
        tensor: QuantizedTensor<Self>,
        dim: usize,
        start: usize,
        length: usize,
    ) -> QuantizedTensor<Self> {
        TchQTensor {
            qtensor: TchOps::narrow(tensor.qtensor, dim, start, length),
            scheme: tensor.scheme,
        }
    }

    fn q_chunk(
        tensor: QuantizedTensor<Self>,
        chunks: usize,
        dim: usize,
    ) -> Vec<QuantizedTensor<Self>> {
        TchOps::chunk(tensor.qtensor, chunks, dim)
            .into_iter()
            .map(|x| TchQTensor {
                qtensor: x,
                scheme: tensor.scheme.clone(),
            })
            .collect()
    }

    fn q_expand(tensor: QuantizedTensor<Self>, shape: Shape) -> QuantizedTensor<Self> {
        // NOTE: with per-channel quantization (future), the channel axis could be impacted by this op
        TchQTensor {
            qtensor: TchOps::expand(tensor.qtensor, shape),
            scheme: tensor.scheme,
        }
    }

    fn q_sort(
        tensor: QuantizedTensor<Self>,
        dim: usize,
        descending: bool,
    ) -> QuantizedTensor<Self> {
        TchQTensor {
            qtensor: TchOps::sort(tensor.qtensor, dim, descending),
            scheme: tensor.scheme,
        }
    }

    fn q_sort_with_indices(
        tensor: QuantizedTensor<Self>,
        dim: usize,
        descending: bool,
    ) -> (QuantizedTensor<Self>, IntTensor<Self>) {
        let (qtensor, indices) = TchOps::sort_with_indices(tensor.qtensor, dim, descending);
        let tensor = TchQTensor {
            qtensor,
            scheme: tensor.scheme,
        };
        (tensor, indices)
    }

    fn q_argsort(tensor: QuantizedTensor<Self>, dim: usize, descending: bool) -> IntTensor<Self> {
        TchOps::argsort(tensor.qtensor, dim, descending)
    }
}
