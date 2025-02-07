use crate::{
    binary_float_cmp_ops, binary_float_ops,
    client::FusionClient,
    get_client,
    ops::binary::check_binary_op_types,
    scalar_float2int_ops, scalar_float_cmp_ops, scalar_float_ops,
    stream::{execution::Operation, StreamId},
    unary_float_ops, Fusion, FusionBackend,
};
use burn_ir::*;
use burn_tensor::{
    ops::{binary_ops_shape, BoolTensor, FloatElem, FloatTensor, FloatTensorOps, IntTensor},
    DType, Device, Distribution, Element, ElementConversion, Shape, TensorData, TensorMetadata,
};
use std::{marker::PhantomData, ops::Range};

use super::NoOp;

impl<B: FusionBackend> FloatTensorOps<Self> for Fusion<B> {
    fn float_from_data(data: TensorData, device: &Device<Self>) -> FloatTensor<Self> {
        let stream = StreamId::current();
        let client = get_client::<B>(&device.clone());
        let dtype = data.dtype;
        let tensor = B::float_from_data(data, device);
        let shape = tensor.shape();

        let handle = B::float_tensor_handle(tensor);
        let out = client.register_tensor(handle, shape.dims, stream, dtype);
        let desc = out.to_tensor_ir_out();

        client.register(
            vec![stream],
            OperationRepr::Init(InitOperationRepr { out: desc }),
            NoOp::<B>::new(),
        );

        out
    }

    fn float_random(
        shape: Shape,
        distribution: Distribution,
        device: &Device<Self>,
    ) -> FloatTensor<Self> {
        #[derive(new)]
        struct RandomOps<B: FusionBackend> {
            desc: RandomOpRepr,
            device: Device<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for RandomOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let shape = Shape::from(self.desc.out.shape.clone());
                let output: B::FloatTensorPrimitive =
                    B::float_random(shape, self.desc.distribution, &self.device);
                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream = StreamId::current();
        let client = get_client::<B>(&device.clone());
        let out = client.tensor_uninitialized(shape.dims, B::FloatElem::dtype());

        let desc = RandomOpRepr {
            out: out.to_tensor_ir_out(),
            distribution,
        };
        client.register(
            vec![stream],
            OperationRepr::Float(
                FloatElem::<Self>::dtype(),
                FloatOperationRepr::Random(desc.clone()),
            ),
            RandomOps::<B>::new(desc, device.clone()),
        );

        out
    }

    fn float_zeros(shape: Shape, device: &Device<Self>) -> FloatTensor<Self> {
        #[derive(new)]
        struct ZerosOps<B: FusionBackend> {
            out: TensorRepr,
            device: Device<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for ZerosOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let shape = Shape::from(self.out.shape.clone());
                let output = B::float_zeros(shape, &self.device);
                handles.register_float_tensor::<B>(&self.out.id, output);
            }
        }

        let stream = StreamId::current();
        let client = get_client::<B>(&device.clone());
        let out = client.tensor_uninitialized(shape.dims, B::FloatElem::dtype());

        let desc = out.to_tensor_ir_out();
        client.register(
            vec![stream],
            OperationRepr::NumericFloat(
                FloatElem::<Self>::dtype(),
                NumericOperationRepr::Zeros(desc.clone()),
            ),
            ZerosOps::<B>::new(desc, device.clone()),
        );

        out
    }

    fn float_ones(shape: Shape, device: &Device<Self>) -> FloatTensor<Self> {
        #[derive(new)]
        struct OnesOps<B: FusionBackend> {
            out: TensorRepr,
            device: Device<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for OnesOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let shape = Shape::from(self.out.shape.clone());
                let output = B::float_ones(shape, &self.device);
                handles.register_float_tensor::<B>(&self.out.id, output);
            }
        }

        let stream = StreamId::current();
        let client = get_client::<B>(&device.clone());
        let out = client.tensor_uninitialized(shape.dims, B::FloatElem::dtype());

        let desc = out.to_tensor_ir_out();
        client.register(
            vec![stream],
            OperationRepr::NumericFloat(
                FloatElem::<Self>::dtype(),
                NumericOperationRepr::Ones(desc.clone()),
            ),
            OnesOps::<B>::new(desc, device.clone()),
        );

        out
    }

    fn float_full(
        shape: Shape,
        fill_value: FloatElem<Self>,
        device: &Device<Self>,
    ) -> FloatTensor<Self> {
        #[derive(new)]
        struct FullOps<B: FusionBackend> {
            out: TensorRepr,
            elem: f32,
            device: Device<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for FullOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let shape = Shape::from(self.out.shape.clone());
                let output: B::FloatTensorPrimitive =
                    B::float_full(shape, self.elem.elem(), &self.device);
                handles.register_float_tensor::<B>(&self.out.id, output);
            }
        }

        let stream = StreamId::current();
        let client = get_client::<B>(&device.clone());
        let out = client.tensor_uninitialized(shape.dims, B::FloatElem::dtype());

        let desc = (out.to_tensor_ir_out(), fill_value.elem::<f32>());
        client.register(
            vec![stream],
            OperationRepr::NumericFloat(
                FloatElem::<Self>::dtype(),
                NumericOperationRepr::Full(desc.clone()),
            ),
            FullOps::<B>::new(desc.0, desc.1, device.clone()),
        );

        out
    }

    async fn float_into_data(tensor: FloatTensor<Self>) -> TensorData {
        tensor.into_data::<B>().await
    }

    fn float_device(tensor: &FloatTensor<Self>) -> Device<Self> {
        tensor.client.device().clone()
    }

    fn float_to_device(tensor: FloatTensor<Self>, device: &Device<Self>) -> FloatTensor<Self> {
        let device_original: &B::Device = tensor.client.device();
        let device_target: B::Device = device.clone();

        if device_original == &device_target {
            return tensor;
        }

        let id = tensor.stream;
        let client_target = get_client::<B>(&device_target);
        let client_original = tensor.client.clone();

        client_original
            .clone()
            .change_client_float::<B>(tensor.into_tensor_ir(), client_target, id)
    }

    fn float_into_int(tensor: FloatTensor<Self>) -> IntTensor<Self> {
        #[derive(new)]
        struct IntoIntOps<B: FusionBackend> {
            desc: UnaryOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for IntoIntOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let input = handles.get_float_tensor::<B>(&self.desc.input);
                let output = B::float_into_int(input);

                handles.register_int_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), B::IntElem::dtype());

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::IntoInt(desc.clone())),
            IntoIntOps::<B>::new(desc),
        );

        out
    }

    fn float_empty(shape: Shape, device: &Device<Self>) -> FloatTensor<Self> {
        #[derive(new)]
        struct EmptyOps<B: FusionBackend> {
            desc: TensorRepr,
            device: Device<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for EmptyOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let output = B::float_empty(Shape::from(&self.desc.shape), &self.device);
                handles.register_float_tensor::<B>(&self.desc.id, output);
            }
        }

        let stream = StreamId::current();
        let client = get_client::<B>(&device.clone());
        let out = client.tensor_uninitialized(shape.dims.clone(), B::FloatElem::dtype());

        let desc = out.to_tensor_ir_out();

        client.register(
            vec![stream],
            OperationRepr::BaseFloat(BaseOperationRepr::Empty(desc.clone())),
            EmptyOps::<B>::new(desc, device.clone()),
        );

        out
    }

    fn float_add(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        binary_float_ops!(AddOps, B::float_add);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), lhs.dtype);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };

        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Add(desc.clone())),
            AddOps::<B>::new(desc),
        );

        out
    }

    fn float_add_scalar(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> FloatTensor<Self> {
        scalar_float_ops!(AddOps, B::float_add_scalar);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs.client.tensor_uninitialized(lhs.shape.clone(), dtype);

        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.elem::<f32>(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::AddScalar(desc.clone())),
            AddOps::<B>::new(desc),
        );

        out
    }

    fn float_clamp(
        tensor: FloatTensor<Self>,
        min: FloatElem<Self>,
        max: FloatElem<Self>,
    ) -> FloatTensor<Self> {
        #[derive(new)]
        struct ClampOps<B: FusionBackend> {
            desc: ClampOpRepr<f32>,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for ClampOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let input = handles.get_float_tensor::<B>(&self.desc.tensor);
                let output = B::float_clamp(input, self.desc.min.elem(), self.desc.max.elem());

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = ClampOpRepr {
            tensor: tensor.into_tensor_ir(),
            min: min.elem(),
            max: max.elem(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Clamp(desc.clone())),
            ClampOps::<B>::new(desc),
        );

        out
    }

    fn float_sub(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        binary_float_ops!(SubOps, B::float_sub);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), lhs.dtype);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Sub(desc.clone())),
            SubOps::<B>::new(desc),
        );

        out
    }

    fn float_sub_scalar(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> FloatTensor<Self> {
        scalar_float_ops!(SubOps, B::float_sub_scalar);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs.client.tensor_uninitialized(lhs.shape.clone(), dtype);
        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.elem(),
            out: out.to_tensor_ir_out(),
        };

        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::SubScalar(desc.clone())),
            SubOps::<B>::new(desc),
        );

        out
    }

    fn float_mul(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        binary_float_ops!(MulOps, B::float_mul);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), lhs.dtype);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Mul(desc.clone())),
            MulOps::<B>::new(desc),
        );

        out
    }

    fn float_mul_scalar(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> FloatTensor<Self> {
        scalar_float_ops!(MulOps, B::float_mul_scalar);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs.client.tensor_uninitialized(lhs.shape.clone(), dtype);

        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.elem(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::MulScalar(desc.clone())),
            MulOps::<B>::new(desc),
        );

        out
    }

    fn float_div(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        binary_float_ops!(DivOps, B::float_div);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), lhs.dtype);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Div(desc.clone())),
            DivOps::<B>::new(desc),
        );

        out
    }

    fn float_div_scalar(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> FloatTensor<Self> {
        scalar_float_ops!(DivOps, B::float_div_scalar);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs.client.tensor_uninitialized(lhs.shape.clone(), dtype);

        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.elem(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::DivScalar(desc.clone())),
            DivOps::<B>::new(desc),
        );

        out
    }

    fn float_remainder(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        binary_float_ops!(ModOps, B::float_remainder);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), lhs.dtype);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Rem(desc.clone())),
            ModOps::<B>::new(desc),
        );

        out
    }

    fn float_remainder_scalar(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> FloatTensor<Self> {
        scalar_float_ops!(ModOps, B::float_remainder_scalar);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs.client.tensor_uninitialized(lhs.shape.clone(), dtype);

        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.elem(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::RemScalar(desc.clone())),
            ModOps::<B>::new(desc),
        );

        out
    }

    fn float_matmul(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        binary_float_ops!(MatmulOps, B::float_matmul);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;
        let mut shape = binary_ops_shape(&lhs.shape, &rhs.shape);
        let ndims = burn_tensor::TensorMetadata::shape(&lhs).num_dims();

        shape[ndims - 2] = lhs.shape[ndims - 2];
        shape[ndims - 1] = rhs.shape[ndims - 1];

        let out = lhs.client.tensor_uninitialized(shape, dtype);
        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };

        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::Float(dtype, FloatOperationRepr::Matmul(desc.clone())),
            MatmulOps::<B>::new(desc),
        );

        out
    }

    fn float_swap_dims(tensor: FloatTensor<Self>, dim1: usize, dim2: usize) -> FloatTensor<Self> {
        #[derive(new)]
        struct SwapDimsOps<B: FusionBackend> {
            desc: SwapDimsOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for SwapDimsOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let input = handles.get_float_tensor::<B>(&self.desc.input);
                let output = B::float_swap_dims(input, self.desc.dim1, self.desc.dim2);
                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let mut shape = tensor.shape.clone();
        shape[dim1] = tensor.shape[dim2];
        shape[dim2] = tensor.shape[dim1];

        let mut out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = SwapDimsOpRepr {
            input: tensor.into_tensor_ir(),
            dim1,
            dim2,
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::BaseFloat(BaseOperationRepr::SwapDims(desc.clone())),
            SwapDimsOps::<B>::new(desc),
        );
        out.stream = stream;

        out
    }

    fn float_reshape(tensor: FloatTensor<Self>, shape: Shape) -> FloatTensor<Self> {
        #[derive(new)]
        struct ReshapeDimsOps<B: FusionBackend> {
            desc: UnaryOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for ReshapeDimsOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let input = handles.get_float_tensor::<B>(&self.desc.input);
                let output = B::float_reshape(input, Shape::from(&self.desc.out.shape));
                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor.client.tensor_uninitialized(shape.dims, dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::BaseFloat(BaseOperationRepr::Reshape(desc.clone())),
            ReshapeDimsOps::<B>::new(desc),
        );

        out
    }

    fn float_gather(
        dim: usize,
        tensor: FloatTensor<Self>,
        indices: IntTensor<Self>,
    ) -> FloatTensor<Self> {
        #[derive(new)]
        struct GatherOps<B: FusionBackend> {
            desc: GatherOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for GatherOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);
                let indices = handles.get_int_tensor::<B>(&self.desc.indices);

                let output = B::float_gather(self.desc.dim, tensor, indices);
                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream_1 = tensor.stream;
        let stream_2 = indices.stream;
        let dtype = tensor.dtype;
        let shape: Vec<usize> = indices.shape.clone();
        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = GatherOpRepr {
            tensor: tensor.into_tensor_ir(),
            dim,
            indices: indices.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Gather(desc.clone())),
            GatherOps::<B>::new(desc),
        );

        out
    }

    fn float_scatter(
        dim: usize,
        tensor: FloatTensor<Self>,
        indices: IntTensor<Self>,
        value: FloatTensor<Self>,
    ) -> FloatTensor<Self> {
        #[derive(new)]
        struct ScatterOps<B: FusionBackend> {
            desc: ScatterOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for ScatterOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);
                let indices = handles.get_int_tensor::<B>(&self.desc.indices);
                let value = handles.get_float_tensor::<B>(&self.desc.value);

                let output = B::float_scatter(self.desc.dim, tensor, indices, value);

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream_1 = tensor.stream;
        let stream_2 = indices.stream;
        let stream_3 = value.stream;
        let dtype = tensor.dtype;
        let shape: Vec<usize> = tensor.shape.clone();
        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = ScatterOpRepr {
            tensor: tensor.into_tensor_ir(),
            dim,
            indices: indices.into_tensor_ir(),
            value: value.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        // Check that both float tensors have the same type
        check_binary_op_types(&desc.tensor, &desc.value).unwrap();
        out.client.register(
            vec![stream_1, stream_2, stream_3],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Scatter(desc.clone())),
            ScatterOps::<B>::new(desc),
        );

        out
    }

    fn float_select(
        tensor: FloatTensor<Self>,
        dim: usize,
        indices: IntTensor<Self>,
    ) -> FloatTensor<Self> {
        #[derive(new)]
        struct SelectOps<B: FusionBackend> {
            desc: SelectOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for SelectOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);
                let indices = handles.get_int_tensor::<B>(&self.desc.indices);

                let output = B::float_select(tensor, self.desc.dim, indices);

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream_1 = tensor.stream;
        let stream_2 = indices.stream;
        let dtype = tensor.dtype;
        let mut shape: Vec<usize> = tensor.shape.clone();
        shape[dim] = indices.shape[0];
        let out = tensor.client.tensor_uninitialized(shape, dtype);
        let desc = SelectOpRepr {
            tensor: tensor.into_tensor_ir(),
            dim,
            indices: indices.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Select(desc.clone())),
            SelectOps::<B>::new(desc),
        );

        out
    }

    fn float_select_assign(
        tensor: FloatTensor<Self>,
        dim: usize,
        indices: IntTensor<Self>,
        value: FloatTensor<Self>,
    ) -> FloatTensor<Self> {
        #[derive(new)]
        struct SelectAssignOps<B: FusionBackend> {
            desc: SelectAssignOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for SelectAssignOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);
                let indices = handles.get_int_tensor::<B>(&self.desc.indices);
                let value = handles.get_float_tensor::<B>(&self.desc.value);

                let output = B::float_select_assign(tensor, self.desc.dim, indices, value);

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream_1 = tensor.stream;
        let stream_2 = indices.stream;
        let stream_3 = value.stream;
        let dtype = tensor.dtype;
        let shape: Vec<usize> = tensor.shape.clone();
        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = SelectAssignOpRepr {
            tensor: tensor.into_tensor_ir(),
            dim,
            indices: indices.into_tensor_ir(),
            value: value.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        // Check that both float tensors have the same type
        check_binary_op_types(&desc.tensor, &desc.value).unwrap();
        out.client.register(
            vec![stream_1, stream_2, stream_3],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::SelectAssign(desc.clone())),
            SelectAssignOps::<B>::new(desc),
        );

        out
    }

    fn float_slice(tensor: FloatTensor<Self>, ranges: &[Range<usize>]) -> FloatTensor<Self> {
        #[derive(new)]
        struct SliceOps<B: FusionBackend> {
            desc: SliceOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for SliceOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);

                let output = B::float_slice(tensor, self.desc.ranges.as_slice());

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }
        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let ndims = burn_tensor::TensorMetadata::shape(&tensor).num_dims();
        let mut shape: Vec<usize> = ranges.iter().map(|range| range.end - range.start).collect();

        for i in shape.len()..ndims {
            shape.push(tensor.shape[i]);
        }

        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = SliceOpRepr {
            tensor: tensor.into_tensor_ir(),
            ranges: ranges.into(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::BaseFloat(BaseOperationRepr::Slice(desc.clone())),
            SliceOps::<B>::new(desc),
        );

        out
    }

    fn float_slice_assign(
        tensor: FloatTensor<Self>,
        ranges: &[Range<usize>],
        value: FloatTensor<Self>,
    ) -> FloatTensor<Self> {
        #[derive(new)]
        struct SliceAssignOps<B: FusionBackend> {
            desc: SliceAssignOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for SliceAssignOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);
                let value = handles.get_float_tensor::<B>(&self.desc.value);

                let output = B::float_slice_assign(tensor, self.desc.ranges.as_slice(), value);

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream_1 = tensor.stream;
        let stream_2 = value.stream;
        let dtype = tensor.dtype;
        let shape: Vec<usize> = tensor.shape.clone();
        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = SliceAssignOpRepr {
            tensor: tensor.into_tensor_ir(),
            ranges: ranges.into(),
            value: value.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        // Check that both float tensors have the same type
        check_binary_op_types(&desc.tensor, &desc.value).unwrap();
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::BaseFloat(BaseOperationRepr::SliceAssign(desc.clone())),
            SliceAssignOps::<B>::new(desc),
        );

        out
    }

    fn float_mask_where(
        tensor: FloatTensor<Self>,
        mask: BoolTensor<Self>,
        value: FloatTensor<Self>,
    ) -> FloatTensor<Self> {
        #[derive(new)]
        struct MaskWhereOps<B: FusionBackend> {
            desc: MaskWhereOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for MaskWhereOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);
                let value = handles.get_float_tensor::<B>(&self.desc.value);
                let mask = handles.get_bool_tensor::<B>(&self.desc.mask);

                let output = B::float_mask_where(tensor, mask, value);

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream_1 = tensor.stream;
        let stream_2 = mask.stream;
        let stream_3 = value.stream;
        let dtype = tensor.dtype;
        let shape = binary_ops_shape(&tensor.shape, &mask.shape);
        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = MaskWhereOpRepr {
            tensor: tensor.into_tensor_ir(),
            value: value.into_tensor_ir(),
            mask: mask.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        // Check that both float tensors have the same type
        check_binary_op_types(&desc.tensor, &desc.value).unwrap();
        out.client.register(
            vec![stream_1, stream_2, stream_3],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::MaskWhere(desc.clone())),
            MaskWhereOps::<B>::new(desc),
        );

        out
    }

    fn float_mask_fill(
        tensor: FloatTensor<Self>,
        mask: BoolTensor<Self>,
        value: FloatElem<Self>,
    ) -> FloatTensor<Self> {
        #[derive(new)]
        struct MaskFillOps<B: FusionBackend> {
            desc: MaskFillOpRepr<f32>,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for MaskFillOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);
                let mask = handles.get_bool_tensor::<B>(&self.desc.mask);

                let output = B::float_mask_fill(tensor, mask, self.desc.value.elem());

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream_1 = tensor.stream;
        let stream_2 = mask.stream;
        let dtype = tensor.dtype;
        let shape: Vec<usize> = tensor.shape.clone();
        let out = tensor.client.tensor_uninitialized(shape, dtype);
        let desc = MaskFillOpRepr {
            tensor: tensor.into_tensor_ir(),
            value: value.elem(),
            mask: mask.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::MaskFill(desc.clone())),
            MaskFillOps::<B>::new(desc),
        );

        out
    }

    fn float_equal(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> BoolTensor<Self> {
        binary_float_cmp_ops!(EqualOps, B::float_equal);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), DType::Bool);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::BaseFloat(BaseOperationRepr::Equal(desc.clone())),
            EqualOps::<B>::new(desc),
        );

        out
    }

    fn float_equal_elem(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> BoolTensor<Self> {
        scalar_float_cmp_ops!(EqualElemOps, B::float_equal_elem);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(lhs.shape.clone(), DType::Bool);

        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.elem(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::EqualElem(desc.clone())),
            EqualElemOps::<B>::new(desc),
        );

        out
    }

    fn float_greater(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> BoolTensor<Self> {
        binary_float_cmp_ops!(GreaterOps, B::float_greater);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), DType::Bool);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Greater(desc.clone())),
            GreaterOps::<B>::new(desc),
        );

        out
    }

    fn float_greater_elem(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> BoolTensor<Self> {
        scalar_float_cmp_ops!(GreaterElemOps, B::float_greater_elem);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(lhs.shape.clone(), DType::Bool);

        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.elem(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::GreaterElem(desc.clone())),
            GreaterElemOps::<B>::new(desc),
        );

        out
    }

    fn float_greater_equal(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> BoolTensor<Self> {
        binary_float_cmp_ops!(GreaterEqualOps, B::float_greater_equal);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), DType::Bool);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::GreaterEqual(desc.clone())),
            GreaterEqualOps::<B>::new(desc),
        );

        out
    }

    fn float_greater_equal_elem(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> BoolTensor<Self> {
        scalar_float_cmp_ops!(GreaterEqualElemOps, B::float_greater_equal_elem);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(lhs.shape.clone(), DType::Bool);

        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.elem(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(
                dtype,
                NumericOperationRepr::GreaterEqualElem(desc.clone()),
            ),
            GreaterEqualElemOps::<B>::new(desc),
        );

        out
    }

    fn float_lower(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> BoolTensor<Self> {
        binary_float_cmp_ops!(LowerOps, B::float_lower);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), DType::Bool);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Lower(desc.clone())),
            LowerOps::<B>::new(desc),
        );

        out
    }

    fn float_lower_elem(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> BoolTensor<Self> {
        scalar_float_cmp_ops!(LowerElemOps, B::float_lower_elem);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(lhs.shape.clone(), DType::Bool);

        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.elem(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::LowerElem(desc.clone())),
            LowerElemOps::<B>::new(desc),
        );

        out
    }

    fn float_lower_equal(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> BoolTensor<Self> {
        binary_float_cmp_ops!(LowerEqualOps, B::float_lower_equal);

        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), DType::Bool);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::LowerEqual(desc.clone())),
            LowerEqualOps::<B>::new(desc),
        );

        out
    }

    fn float_lower_equal_elem(lhs: FloatTensor<Self>, rhs: FloatElem<Self>) -> BoolTensor<Self> {
        scalar_float_cmp_ops!(LowerEqualElemOps, B::float_lower_equal_elem);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs
            .client
            .tensor_uninitialized(lhs.shape.clone(), DType::Bool);

        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.elem(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::LowerEqualElem(desc.clone())),
            LowerEqualElemOps::<B>::new(desc),
        );

        out
    }

    fn float_sum(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(SumOps, B::float_sum, reduce);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor.client.tensor_uninitialized(vec![1], dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Sum(desc.clone())),
            SumOps::<B>::new(desc),
        );

        out
    }

    fn float_sum_dim(tensor: FloatTensor<Self>, dim: usize) -> FloatTensor<Self> {
        scalar_float_ops!(SumDimOps, B::float_sum_dim, usize, noconvert);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let mut shape = tensor.shape.clone();
        shape[dim] = 1;
        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = ScalarOpRepr {
            lhs: tensor.into_tensor_ir(),
            rhs: dim,
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::SumDim(desc.clone())),
            SumDimOps::<B>::new(desc),
        );

        out
    }

    fn float_prod(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(ProdOps, B::float_prod, reduce);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor.client.tensor_uninitialized(vec![1], dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Prod(desc.clone())),
            ProdOps::<B>::new(desc),
        );

        out
    }

    fn float_prod_dim(tensor: FloatTensor<Self>, dim: usize) -> FloatTensor<Self> {
        scalar_float_ops!(ProdDimOps, B::float_prod_dim, usize, noconvert);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let mut shape = tensor.shape.clone();
        shape[dim] = 1;
        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = ScalarOpRepr {
            lhs: tensor.into_tensor_ir(),
            rhs: dim,
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(
                FloatElem::<Self>::dtype(),
                NumericOperationRepr::ProdDim(desc.clone()),
            ),
            ProdDimOps::<B>::new(desc),
        );

        out
    }

    fn float_mean(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(MeanOps, B::float_mean, reduce);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor.client.tensor_uninitialized(vec![1], dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Mean(desc.clone())),
            MeanOps::<B>::new(desc),
        );

        out
    }

    fn float_mean_dim(tensor: FloatTensor<Self>, dim: usize) -> FloatTensor<Self> {
        scalar_float_ops!(MeanDimOps, B::float_mean_dim, usize, noconvert);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let mut shape = tensor.shape.clone();
        shape[dim] = 1;
        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = ScalarOpRepr {
            lhs: tensor.into_tensor_ir(),
            rhs: dim,
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::MeanDim(desc.clone())),
            MeanDimOps::<B>::new(desc),
        );

        out
    }

    fn float_exp(lhs: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(ExpOps, B::float_exp);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs.client.tensor_uninitialized(lhs.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: lhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Exp(desc.clone())),
            ExpOps::<B>::new(desc),
        );

        out
    }

    fn float_log(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(LogOps, B::float_log);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Log(desc.clone())),
            LogOps::<B>::new(desc),
        );

        out
    }

    fn float_log1p(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(Log1pOps, B::float_log1p);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Log1p(desc.clone())),
            Log1pOps::<B>::new(desc),
        );

        out
    }

    fn float_powf_scalar(lhs: FloatTensor<Self>, rhs: f32) -> FloatTensor<Self> {
        scalar_float_ops!(PowfOps, B::float_powf_scalar, f32);

        let stream = lhs.stream;
        let dtype = lhs.dtype;
        let out = lhs.client.tensor_uninitialized(lhs.shape.clone(), dtype);

        let desc = ScalarOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs,
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::PowfScalar(desc.clone())),
            PowfOps::<B>::new(desc),
        );

        out
    }

    fn float_sqrt(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(SqrtOps, B::float_sqrt);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Sqrt(desc.clone())),
            SqrtOps::<B>::new(desc),
        );

        out
    }

    fn float_abs(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(AbsOps, B::float_abs);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Abs(desc.clone())),
            AbsOps::<B>::new(desc),
        );

        out
    }

    fn float_cos(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(CosOps, B::float_cos);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Cos(desc.clone())),
            CosOps::<B>::new(desc),
        );

        out
    }

    fn float_sin(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(SinOps, B::float_sin);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Sin(desc.clone())),
            SinOps::<B>::new(desc),
        );

        out
    }

    fn float_tanh(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(TanhOps, B::float_tanh);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Tanh(desc.clone())),
            TanhOps::<B>::new(desc),
        );

        out
    }

    fn float_recip(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(Recip, B::float_recip);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);
        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Recip(desc.clone())),
            Recip::<B>::new(desc),
        );

        out
    }

    fn float_erf(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(TanhOps, B::float_erf);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Erf(desc.clone())),
            TanhOps::<B>::new(desc),
        );

        out
    }

    fn float_cat(tensors: Vec<FloatTensor<Self>>, dim: usize) -> FloatTensor<Self> {
        #[derive(new)]
        struct CatOps<B: FusionBackend> {
            desc: CatOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for CatOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensors = self
                    .desc
                    .tensors
                    .iter()
                    .map(|tensor| handles.get_float_tensor::<B>(tensor))
                    .collect();

                let output = B::float_cat(tensors, self.desc.dim);

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let tensor_first = tensors.first().unwrap();
        let dtype = tensor_first.dtype;
        let client = tensor_first.client.clone();

        // Calculate the output shape
        let streams = tensors.iter().map(|tensor| tensor.stream).collect();
        let mut shape: Vec<usize> = tensor_first.shape.clone();
        shape[dim] = 0;
        for tensor in tensors.iter() {
            shape[dim] += tensor.shape[dim];
        }

        let out = client.tensor_uninitialized(shape, dtype);

        let desc = CatOpRepr {
            tensors: tensors.into_iter().map(|t| t.into_tensor_ir()).collect(),
            dim,
            out: out.to_tensor_ir_out(),
        };
        desc.tensors
            .windows(2)
            .for_each(|desc| check_binary_op_types(&desc[0], &desc[1]).unwrap());
        client.register(
            streams,
            OperationRepr::BaseFloat(BaseOperationRepr::Cat(desc.clone())),
            CatOps::<B>::new(desc),
        );

        out
    }

    fn float_argmax(tensor: FloatTensor<Self>, dim: usize) -> IntTensor<Self> {
        scalar_float2int_ops!(ArgMaxOps, B::float_argmax, usize);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let mut shape = tensor.shape.clone();
        shape[dim] = 1;
        let out = tensor
            .client
            .tensor_uninitialized(shape, B::IntElem::dtype());

        let desc = ScalarOpRepr {
            lhs: tensor.into_tensor_ir(),
            rhs: dim,
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::ArgMax(desc.clone())),
            ArgMaxOps::<B>::new(desc),
        );

        out
    }

    fn float_repeat_dim(tensor: FloatTensor<Self>, dim: usize, times: usize) -> FloatTensor<Self> {
        #[derive(new)]
        struct RepeatDimOps<B: FusionBackend> {
            desc: RepeatDimOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for RepeatDimOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);

                let output = B::float_repeat_dim(tensor, self.desc.dim, self.desc.times);

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream = tensor.stream;
        let mut shape = tensor.shape.clone();
        shape[dim] *= times;
        let out = tensor.client.tensor_uninitialized(shape, tensor.dtype);

        let desc = RepeatDimOpRepr {
            tensor: tensor.into_tensor_ir(),
            dim,
            times,
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::BaseFloat(BaseOperationRepr::RepeatDim(desc.clone())),
            RepeatDimOps::<B>::new(desc),
        );

        out
    }

    fn float_argmin(tensor: FloatTensor<Self>, dim: usize) -> IntTensor<Self> {
        scalar_float2int_ops!(ArgMinOps, B::float_argmin, usize);

        let stream = tensor.stream;
        let mut shape = tensor.shape.clone();
        shape[dim] = 1;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(shape, B::IntElem::dtype());

        let desc = ScalarOpRepr {
            lhs: tensor.into_tensor_ir(),
            rhs: dim,
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::ArgMin(desc.clone())),
            ArgMinOps::<B>::new(desc),
        );

        out
    }

    fn float_max(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(MaxOps, B::float_max, reduce);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor.client.tensor_uninitialized(vec![1], dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Max(desc.clone())),
            MaxOps::<B>::new(desc),
        );

        out
    }

    fn float_max_dim(tensor: FloatTensor<Self>, dim: usize) -> FloatTensor<Self> {
        scalar_float_ops!(MaxDimOps, B::float_max_dim, usize, noconvert);

        let stream = tensor.stream;
        let mut shape = tensor.shape.clone();
        let dtype = tensor.dtype;
        shape[dim] = 1;
        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = ScalarOpRepr {
            lhs: tensor.into_tensor_ir(),
            rhs: dim,
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::MaxDim(desc.clone())),
            MaxDimOps::<B>::new(desc),
        );

        out
    }

    fn float_max_dim_with_indices(
        tensor: FloatTensor<Self>,
        dim: usize,
    ) -> (FloatTensor<Self>, IntTensor<Self>) {
        #[derive(new)]
        struct MaxDimWithIndicesOps<B: FusionBackend> {
            desc: ReduceDimWithIndicesOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for MaxDimWithIndicesOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);
                let (output, indices) = B::float_max_dim_with_indices(tensor, self.desc.dim);

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
                handles.register_int_tensor::<B>(&self.desc.out_indices.id, indices);
            }
        }

        let stream = tensor.stream;
        let mut shape = tensor.shape.clone();
        shape[dim] = 1;
        let dtype = tensor.dtype;
        let client = tensor.client.clone();
        let out = client.tensor_uninitialized(shape.clone(), dtype);
        let out_indices = client.tensor_uninitialized(shape, B::IntElem::dtype());

        let desc = ReduceDimWithIndicesOpRepr {
            tensor: tensor.into_tensor_ir(),
            dim,
            out: out.to_tensor_ir_out(),
            out_indices: out_indices.to_tensor_ir_out(),
        };
        client.register(
            vec![stream],
            OperationRepr::NumericFloat(
                dtype,
                NumericOperationRepr::MaxDimWithIndices(desc.clone()),
            ),
            MaxDimWithIndicesOps::<B>::new(desc),
        );

        (out, out_indices)
    }

    fn float_min(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(MinOps, B::float_min, reduce);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor.client.tensor_uninitialized(vec![1], dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Min(desc.clone())),
            MinOps::<B>::new(desc),
        );

        out
    }

    fn float_min_dim(tensor: FloatTensor<Self>, dim: usize) -> FloatTensor<Self> {
        scalar_float_ops!(MinDimOps, B::float_min_dim, usize, noconvert);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let mut shape = tensor.shape.clone();
        shape[dim] = 1;
        let out = tensor.client.tensor_uninitialized(shape, dtype);

        let desc = ScalarOpRepr {
            lhs: tensor.into_tensor_ir(),
            rhs: dim,
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::MinDim(desc.clone())),
            MinDimOps::<B>::new(desc),
        );

        out
    }

    fn float_min_dim_with_indices(
        tensor: FloatTensor<Self>,
        dim: usize,
    ) -> (FloatTensor<Self>, IntTensor<Self>) {
        #[derive(new)]
        struct MinDimWithIndicesOps<B: FusionBackend> {
            desc: ReduceDimWithIndicesOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for MinDimWithIndicesOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let tensor = handles.get_float_tensor::<B>(&self.desc.tensor);
                let (output, indices) = B::float_min_dim_with_indices(tensor, self.desc.dim);

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
                handles.register_int_tensor::<B>(&self.desc.out_indices.id, indices);
            }
        }

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let mut shape = tensor.shape.clone();
        shape[dim] = 1;
        let client = tensor.client.clone();
        let out = client.tensor_uninitialized(shape.clone(), dtype);
        let out_indices = client.tensor_uninitialized(shape, B::IntElem::dtype());

        let desc = ReduceDimWithIndicesOpRepr {
            tensor: tensor.into_tensor_ir(),
            dim,
            out: out.to_tensor_ir_out(),
            out_indices: out_indices.to_tensor_ir_out(),
        };
        client.register(
            vec![stream],
            OperationRepr::NumericFloat(
                dtype,
                NumericOperationRepr::MinDimWithIndices(desc.clone()),
            ),
            MinDimWithIndicesOps::<B>::new(desc),
        );

        (out, out_indices)
    }

    fn float_powf(lhs: FloatTensor<Self>, rhs: FloatTensor<Self>) -> FloatTensor<Self> {
        binary_float_ops!(PowOps, B::float_powf);
        let stream_1 = lhs.stream;
        let stream_2 = rhs.stream;
        let dtype = lhs.dtype;

        let out = lhs
            .client
            .tensor_uninitialized(binary_ops_shape(&lhs.shape, &rhs.shape), dtype);

        let desc = BinaryOpRepr {
            lhs: lhs.into_tensor_ir(),
            rhs: rhs.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream_1, stream_2],
            OperationRepr::NumericFloat(dtype, NumericOperationRepr::Powf(desc.clone())),
            PowOps::<B>::new(desc),
        );

        out
    }

    fn float_permute(tensor: FloatTensor<Self>, axes: &[usize]) -> FloatTensor<Self> {
        #[derive(new)]
        struct PermuteDimsOps<B: FusionBackend> {
            desc: PermuteOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for PermuteDimsOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let input = handles.get_float_tensor::<B>(&self.desc.input);
                let output = B::float_permute(input, self.desc.axes.as_slice());
                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream = tensor.stream;

        // Change the shape of the tensor to match the new axes
        let shape = axes.iter().map(|x| tensor.shape[*x]).collect();

        let out = tensor.client.tensor_uninitialized(shape, tensor.dtype);

        let desc = PermuteOpRepr {
            input: tensor.into_tensor_ir(),
            axes: axes.to_vec(),
            out: out.to_tensor_ir_out(),
        };

        out.client.register(
            vec![stream],
            OperationRepr::BaseInt(BaseOperationRepr::Permute(desc.clone())),
            PermuteDimsOps::<B>::new(desc),
        );

        out
    }

    fn float_expand(tensor: FloatTensor<Self>, shape: Shape) -> FloatTensor<Self> {
        #[derive(new)]
        struct ExpandOps<B: FusionBackend> {
            desc: ExpandOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for ExpandOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let input = handles.get_float_tensor::<B>(&self.desc.input);
                let output = B::float_expand(input, self.desc.shape.into());

                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream = tensor.stream;

        let out = tensor
            .client
            .tensor_uninitialized(shape.dims.clone(), tensor.dtype);

        let desc = ExpandOpRepr {
            input: tensor.into_tensor_ir(),
            shape: shape.dims,
            out: out.to_tensor_ir_out(),
        };

        out.client.register(
            vec![stream],
            OperationRepr::BaseFloat(BaseOperationRepr::Expand(desc.clone())),
            ExpandOps::<B>::new(desc),
        );

        out
    }

    fn float_flip(tensor: FloatTensor<Self>, axes: &[usize]) -> FloatTensor<Self> {
        #[derive(new)]
        struct FlipOps<B: FusionBackend> {
            desc: FlipOpRepr,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for FlipOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let input = handles.get_float_tensor::<B>(&self.desc.input);
                let output = B::float_flip(input, &self.desc.axes);
                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream = tensor.stream;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), tensor.dtype);

        let desc = FlipOpRepr {
            input: tensor.into_tensor_ir(),
            axes: axes.to_vec(),
            out: out.to_tensor_ir_out(),
        };

        out.client.register(
            vec![stream],
            OperationRepr::BaseInt(BaseOperationRepr::Flip(desc.clone())),
            FlipOps::<B>::new(desc),
        );

        out
    }

    fn float_round(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(RoundOps, B::float_round);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Round(desc.clone())),
            RoundOps::<B>::new(desc),
        );

        out
    }

    fn float_floor(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(FloorOps, B::float_floor);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Floor(desc.clone())),
            FloorOps::<B>::new(desc),
        );

        out
    }

    fn float_ceil(tensor: FloatTensor<Self>) -> FloatTensor<Self> {
        unary_float_ops!(CeilOps, B::float_ceil);

        let stream = tensor.stream;
        let dtype = tensor.dtype;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype);

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::Float(dtype, FloatOperationRepr::Ceil(desc.clone())),
            CeilOps::<B>::new(desc),
        );

        out
    }

    fn float_cast(tensor: FloatTensor<Self>, dtype: burn_tensor::FloatDType) -> FloatTensor<Self> {
        #[derive(new)]
        struct CastOps<B: FusionBackend> {
            desc: UnaryOpRepr,
            dtype: burn_tensor::FloatDType,
            _b: PhantomData<B>,
        }

        impl<B: FusionBackend> Operation<B::FusionRuntime> for CastOps<B> {
            fn execute(self: Box<Self>, handles: &mut HandleContainer<B::Handle>) {
                let input = handles.get_float_tensor::<B>(&self.desc.input);
                let output: B::FloatTensorPrimitive = B::float_cast(input, self.dtype);
                handles.register_float_tensor::<B>(&self.desc.out.id, output);
            }
        }

        let stream = tensor.stream;
        let out = tensor
            .client
            .tensor_uninitialized(tensor.shape.clone(), dtype.clone().into());

        let desc = UnaryOpRepr {
            input: tensor.into_tensor_ir(),
            out: out.to_tensor_ir_out(),
        };
        out.client.register(
            vec![stream],
            OperationRepr::BaseFloat(BaseOperationRepr::Cast(desc.clone())),
            CastOps::<B>::new(desc, dtype),
        );

        out
    }
}
