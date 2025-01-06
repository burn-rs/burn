use super::{expand, numeric, permute};
use crate::kernel::prng::{random_bernoulli, random_normal, random_uniform};
use crate::kernel::{self, launch_unary_float, reduce, FloatUnaryOp, FloatUnaryOpFamily};
use crate::{
    element::BoolElement,
    kernel::matmul::{matmul, MatmulStrategy},
};
use crate::{execute_with_dtype, JitBackend};
use crate::{FloatElement, IntElement, JitRuntime};
use burn_tensor::ops::{BoolTensor, Device, FloatElem, FloatTensor, IntTensor};
use burn_tensor::{ops::FloatTensorOps, Distribution, Shape, TensorData};
use burn_tensor::{DType, ElementConversion, FloatDType};
use cubecl::prelude::*;
use half::{bf16, f16};
use std::ops::Range;

impl<R, F, I, BT> FloatTensorOps<Self> for JitBackend<R, F, I, BT>
where
    R: JitRuntime,
    F: FloatElement,
    I: IntElement,
    BT: BoolElement,
{
    fn float_from_data(data: TensorData, device: &Device<Self>) -> FloatTensor<Self> {
        super::from_data::<R, F>(data, device)
    }

    fn float_random(
        shape: Shape,
        distribution: Distribution,
        device: &Device<Self>,
    ) -> FloatTensor<Self> {
        match distribution {
            Distribution::Default => random_uniform(shape, device, 0.elem::<F>(), 1.elem()),
            Distribution::Uniform(low, high) => {
                random_uniform(shape, device, low.elem::<F>(), high.elem())
            }
            Distribution::Bernoulli(prob) => random_bernoulli(shape, device, prob.elem::<F>()),
            Distribution::Normal(mean, std) => {
                random_normal(shape, device, mean.elem::<F>(), std.elem())
            }
        }
    }

    async fn float_into_data(tensor: FloatTensor<Self>) -> TensorData {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            super::into_data::<R, E>(tensor).await
        )
    }

    fn float_device(tensor: &FloatTensor<Self>) -> Device<Self> {
        tensor.device.clone()
    }

    fn float_to_device(tensor: FloatTensor<Self>, device: &Device<Self>) -> FloatTensor<Self> {
        super::to_device(tensor, device)
    }

    fn float_empty(shape: Shape, device: &Device<Self>) -> FloatTensor<Self> {
        super::empty::<R, F>(shape, device)
    }

    fn float_add(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            numeric::add::<R, E>(lhs, rhs)
        )
    }

    fn float_add_scalar(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype),
            E,
            numeric::add_scalar::<R, E>(lhs, rhs.elem())
        )
    }

    fn float_zeros(shape: Shape, device: &Device<Self>) -> FloatTensor<Self> {
        numeric::zeros::<R, F>(shape, device)
    }

    fn float_full(
        shape: Shape,
        fill_value: FloatElem<Self>,
        device: &R::Device,
    ) -> FloatTensor<Self> {
        numeric::full::<R, F>(shape, device, fill_value)
    }

    fn float_ones(shape: Shape, device: &Device<Self>) -> FloatTensor<Self> {
        numeric::ones::<R, F>(shape, device)
    }

    fn float_sub(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            numeric::sub::<R, E>(lhs, rhs)
        )
    }

    fn float_sub_scalar(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype),
            E,
            numeric::sub_scalar::<R, E>(lhs, rhs.elem())
        )
    }

    fn float_mul(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            numeric::mul::<R, E>(lhs, rhs)
        )
    }

    fn float_mul_scalar(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype),
            E,
            numeric::mul_scalar::<R, E>(lhs, rhs.elem())
        )
    }

    fn float_div(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            numeric::div::<R, E>(lhs, rhs)
        )
    }

    fn float_div_scalar(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype),
            E,
            numeric::div_scalar::<R, E>(lhs, rhs.elem())
        )
    }

    fn float_remainder(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            numeric::remainder::<R, E>(lhs, rhs)
        )
    }

    fn float_remainder_scalar(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype),
            E,
            numeric::remainder_scalar::<R, E>(lhs, rhs.elem())
        )
    }

    fn float_matmul(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            matmul::<R, E>(lhs, rhs, None, MatmulStrategy::default())
        )
    }

    fn float_swap_dims(tensor: FloatTensor<Self>, dim1: usize, dim2: usize) -> FloatTensor<Self> {
        super::swap_dims(tensor, dim1, dim2)
    }

    fn float_reshape(tensor: FloatTensor<Self>, shape: Shape) -> FloatTensor<Self> {
        super::reshape(tensor, shape)
    }

    fn float_gather(
        dim: usize,
        tensor: FloatTensor<Self>,
        indices: IntTensor<Self>,
    ) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            kernel::gather::<R, E, I>(dim, tensor, indices)
        )
    }

    fn float_scatter(
        dim: usize,
        tensor: FloatTensor<Self>,
        indices: IntTensor<Self>,
        value: FloatTensor<Self>,
    ) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype, value.dtype),
            E,
            kernel::scatter::<R, E, I>(dim, tensor, indices, value)
        )
    }

    fn float_select(
        tensor: FloatTensor<Self>,
        dim: usize,
        indices: IntTensor<Self>,
    ) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            kernel::select::<R, E, I>(tensor, dim, indices)
        )
    }

    fn float_select_assign(
        tensor: FloatTensor<Self>,
        dim: usize,
        indices: IntTensor<Self>,
        value: FloatTensor<Self>,
    ) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype, value.dtype),
            E,
            kernel::select_assign::<R, E, I>(tensor, dim, indices, value)
        )
    }

    fn float_slice(tensor: FloatTensor<Self>, ranges: &[Range<usize>]) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            kernel::slice::<R, E>(tensor, ranges)
        )
    }

    fn float_slice_assign(
        tensor: FloatTensor<Self>,
        ranges: &[Range<usize>],
        value: FloatTensor<Self>,
    ) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype, value.dtype),
            E,
            kernel::slice_assign::<R, E>(tensor, ranges, value)
        )
    }

    fn float_mask_where(
        tensor: FloatTensor<Self>,
        mask: BoolTensor<Self>,
        value: FloatTensor<Self>,
    ) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype, value.dtype),
            E,
            kernel::mask_where_auto::<R, E, BT>(tensor, mask, value)
        )
    }

    fn float_mask_fill(
        tensor: FloatTensor<Self>,
        mask: BoolTensor<Self>,
        value: FloatElem<Self>,
    ) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            kernel::mask_fill_auto::<R, E, BT>(tensor, mask, value.elem())
        )
    }

    fn float_equal(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> BoolTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            kernel::equal::<R, E, BT>(lhs, rhs)
        )
    }

    fn float_equal_elem(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> BoolTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype),
            E,
            kernel::equal_elem::<R, E, BT>(lhs, rhs.elem())
        )
    }

    fn float_greater(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> BoolTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            kernel::greater::<R, E, BT>(lhs, rhs)
        )
    }

    fn float_greater_elem(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> BoolTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype),
            E,
            kernel::greater_elem::<R, E, BT>(lhs, rhs.elem())
        )
    }

    fn float_greater_equal(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> BoolTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            kernel::greater_equal::<R, E, BT>(lhs, rhs)
        )
    }

    fn float_greater_equal_elem(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> BoolTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype),
            E,
            kernel::greater_equal_elem::<R, E, BT>(lhs, rhs.elem())
        )
    }

    fn float_lower(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> BoolTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            kernel::lower::<R, E, BT>(lhs, rhs)
        )
    }

    fn float_lower_elem(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> BoolTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype),
            E,
            kernel::lower_elem::<R, E, BT>(lhs, rhs.elem())
        )
    }

    fn float_lower_equal(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> BoolTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype, rhs.dtype),
            E,
            kernel::lower_equal::<R, E, BT>(lhs, rhs)
        )
    }

    fn float_lower_equal_elem(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> BoolTensor<Self> {
        execute_with_dtype!(
            float(lhs.dtype),
            E,
            kernel::lower_equal_elem::<R, E, BT>(lhs, rhs.elem())
        )
    }

    fn float_sum(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            reduce::sum::<R, E>(tensor, Default::default())
        )
    }

    fn float_sum_dim(tensor: FloatTensor<Self>, dim: usize) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            reduce::sum_dim::<R, E, E>(tensor, dim, Default::default())
        )
    }

    fn float_mean_dim(tensor: FloatTensor<Self>, dim: usize) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            reduce::mean_dim::<R, E, E>(tensor, dim, Default::default())
        )
    }

    fn float_prod(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            reduce::prod::<R, E>(tensor, Default::default())
        )
    }

    fn float_prod_dim(tensor: FloatTensor<Self>, dim: usize) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            reduce::prod_dim::<R, E, E>(tensor, dim, Default::default())
        )
    }

    fn float_exp(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Exp;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Exp {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::exp(input)
            }
        }

        impl FloatUnaryOpFamily for Exp {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Exp, _>(tensor, |_| ())
        )
    }

    fn float_log(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Log;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Log {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::log(input)
            }
        }

        impl FloatUnaryOpFamily for Log {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Log, _>(tensor, |_| ())
        )
    }

    fn float_log1p(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Log1p;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Log1p {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::log1p(input)
            }
        }

        impl FloatUnaryOpFamily for Log1p {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Log1p, _>(tensor, |_| ())
        )
    }

    fn float_powf_scalar(lhs: FloatTensor<Self>, rhs: f32) -> FloatTensor<Self> {
        struct Powf;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Powf {
            type Options = F;

            fn execute(input: Line<F>, options: &Self::Options) -> Line<F> {
                Line::powf(input, Line::new(*options))
            }
        }

        impl FloatUnaryOpFamily for Powf {
            type Options<F: Float> = F;
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(lhs.dtype),
            F,
            launch_unary_float::<R, F, Powf, _>(lhs, |_| ScalarArg::new(rhs.elem::<F>()))
        )
    }

    fn float_sqrt(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Sqrt;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Sqrt {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::sqrt(input)
            }
        }

        impl FloatUnaryOpFamily for Sqrt {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Sqrt, _>(tensor, |_| ())
        )
    }

    fn float_abs(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Abs;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Abs {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::abs(input)
            }
        }

        impl FloatUnaryOpFamily for Abs {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Abs, _>(tensor, |_| ())
        )
    }

    fn float_cos(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Cos;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Cos {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::cos(input)
            }
        }

        impl FloatUnaryOpFamily for Cos {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Cos, _>(tensor, |_| ())
        )
    }

    fn float_sin(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Sin;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Sin {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::sin(input)
            }
        }

        impl FloatUnaryOpFamily for Sin {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Sin, _>(tensor, |_| ())
        )
    }

    fn float_tanh(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Tanh;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Tanh {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::tanh(input)
            }
        }

        impl FloatUnaryOpFamily for Tanh {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Tanh, _>(tensor, |_| ())
        )
    }

    fn float_round(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Round;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Round {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::round(input)
            }
        }

        impl FloatUnaryOpFamily for Round {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Round, _>(tensor, |_| ())
        )
    }

    fn float_floor(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Floor;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Floor {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::floor(input)
            }
        }

        impl FloatUnaryOpFamily for Floor {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Floor, _>(tensor, |_| ())
        )
    }

    fn float_ceil(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Ceil;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Ceil {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::ceil(input)
            }
        }

        impl FloatUnaryOpFamily for Ceil {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Ceil, _>(tensor, |_| ())
        )
    }

    fn float_erf(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Erf;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Erf {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::erf(input)
            }
        }

        impl FloatUnaryOpFamily for Erf {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Erf, _>(tensor, |_| ())
        )
    }

    fn float_argmax(tensor: FloatTensor<Self>, dim: usize) -> IntTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            reduce::argmax::<R, E, I>(tensor, dim, Default::default())
        )
    }

    fn float_argmin(tensor: FloatTensor<Self>, dim: usize) -> IntTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            reduce::argmin::<R, E, I>(tensor, dim, Default::default())
        )
    }

    fn float_into_int(tensor: FloatTensor<Self>) -> IntTensor<Self> {
        execute_with_dtype!(float(tensor.dtype), E, kernel::cast::<R, E, I>(tensor))
    }

    fn float_clamp(
        tensor: FloatTensor<Self>,
        min: FloatElem<Self>,
        max: FloatElem<Self>,
    ) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            kernel::clamp::<R, E>(tensor, min.elem(), max.elem())
        )
    }

    fn float_recip(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        struct Recip;

        #[cube]
        impl<F: Float> FloatUnaryOp<F> for Recip {
            type Options = ();

            fn execute(input: Line<F>, _options: &Self::Options) -> Line<F> {
                Line::recip(input)
            }
        }

        impl FloatUnaryOpFamily for Recip {
            type Options<F: Float> = ();
            type Unary<F: Float> = Self;
        }

        execute_with_dtype!(
            float(tensor.dtype),
            F,
            launch_unary_float::<R, F, Recip, _>(tensor, |_| ())
        )
    }

    fn float_repeat_dim(tensor: FloatTensor<Self>, dim: usize, times: usize) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            kernel::repeat_dim::<R, E>(tensor, dim, times)
        )
    }

    fn float_powf(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        execute_with_dtype!(float(lhs.dtype), E, numeric::pow::<R, E>(lhs, rhs))
    }

    fn float_permute(tensor: FloatTensor<Self>, axes: &[usize]) -> FloatTensor<Self> {
        permute(tensor, axes)
    }

    fn float_expand(tensor: FloatTensor<Self>, shape: Shape) -> FloatTensor<Self> {
        expand(tensor, shape)
    }

    fn float_flip(tensor: FloatTensor<Self>, axes: &[usize]) -> FloatTensor<Self> {
        execute_with_dtype!(
            float(tensor.dtype),
            E,
            kernel::flip::<R, E, BT>(tensor, axes)
        )
    }

    fn float_cast(tensor: FloatTensor<Self>, dtype: FloatDType) -> FloatTensor<Self> {
        match (tensor.dtype, dtype) {
            (DType::F64, FloatDType::F64)
            | (DType::F32, FloatDType::F32)
            | (DType::BF16, FloatDType::BF16)
            | (DType::F16, FloatDType::F16) => tensor,
            (DType::F64, FloatDType::F32) => kernel::cast::<R, f64, f32>(tensor),
            (DType::F64, FloatDType::F16) => kernel::cast::<R, f64, f16>(tensor),
            (DType::F64, FloatDType::BF16) => kernel::cast::<R, f64, bf16>(tensor),
            (DType::F32, FloatDType::F64) => kernel::cast::<R, f32, f64>(tensor),
            (DType::F32, FloatDType::F16) => kernel::cast::<R, f32, f16>(tensor),
            (DType::F32, FloatDType::BF16) => kernel::cast::<R, f32, bf16>(tensor),
            (DType::F16, FloatDType::F64) => kernel::cast::<R, f16, f64>(tensor),
            (DType::F16, FloatDType::F32) => kernel::cast::<R, f16, f32>(tensor),
            (DType::F16, FloatDType::BF16) => kernel::cast::<R, f16, bf16>(tensor),
            (DType::BF16, FloatDType::F64) => kernel::cast::<R, bf16, f64>(tensor),
            (DType::BF16, FloatDType::F32) => kernel::cast::<R, bf16, f32>(tensor),
            (DType::BF16, FloatDType::F16) => kernel::cast::<R, bf16, f16>(tensor),
            _ => unimplemented!("Unsupported floating point type cast"),
        }
    }
}
