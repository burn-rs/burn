use crate::{
    context::WorkGroup,
    element::WGPUElement,
    kernel::{build_binary_info, KernelSettings},
    kernel_wgsl,
    pool::get_context,
    tensor::WGPUTensor,
    GraphicsAPI, WGPUDevice,
};
use burn_tensor::{backend::Backend, Data, Shape};
use std::{marker::PhantomData, sync::Arc};

pub type FloatElem<B> = <B as Backend>::FloatElem;
pub type Device<B> = <B as Backend>::Device;

pub type FloatTensor<B, const D: usize> = <B as Backend>::TensorPrimitive<D>;

pub type IntElem<B> = <B as Backend>::IntElem;
pub type IntTensor<B, const D: usize> = <B as Backend>::IntTensorPrimitive<D>;
pub type BoolTensor<B, const D: usize> = <B as Backend>::BoolTensorPrimitive<D>;

pub struct BaseOps<G: GraphicsAPI> {
    _g: PhantomData<G>,
}

impl<G: GraphicsAPI> BaseOps<G> {
    pub fn from_data<E: WGPUElement, const D: usize>(
        data: Data<E, D>,
        device: &WGPUDevice,
    ) -> WGPUTensor<E, D> {
        let context = get_context::<G>(device);
        let buffer = context.create_buffer_with_data(E::as_bytes(&data.value));

        WGPUTensor::new(context, data.shape, Arc::new(buffer))
    }

    pub fn into_data<E: WGPUElement, const D: usize>(tensor: WGPUTensor<E, D>) -> Data<E, D> {
        let tensor = Self::into_continuous(tensor);
        let bytes = tensor.context.buffer_to_data(&tensor.buffer);
        let values = E::from_bytes(&bytes);

        Data::new(values.to_vec(), tensor.shape)
    }

    pub fn to_device<E: WGPUElement, const D: usize>(
        tensor: WGPUTensor<E, D>,
        device: &WGPUDevice,
    ) -> WGPUTensor<E, D> {
        if &tensor.context.device == device {
            return tensor;
        }

        let context = get_context::<G>(device);
        tensor.to_context(context)
    }

    pub fn empty<E: WGPUElement, const D: usize>(
        shape: Shape<D>,
        device: &WGPUDevice,
    ) -> WGPUTensor<E, D> {
        let context = get_context::<G>(device);
        let buffer = context.create_buffer(shape.num_elements() * core::mem::size_of::<E>());

        WGPUTensor::new(context, shape, Arc::new(buffer))
    }

    pub fn swap_dims<E: WGPUElement, const D: usize>(
        mut tensor: WGPUTensor<E, D>,
        dim1: usize,
        dim2: usize,
    ) -> WGPUTensor<E, D> {
        let tmp = tensor.strides[dim1];
        tensor.strides[dim1] = tensor.strides[dim2];
        tensor.strides[dim2] = tmp;

        let tmp = tensor.shape.dims[dim1];
        tensor.shape.dims[dim1] = tensor.shape.dims[dim2];
        tensor.shape.dims[dim2] = tmp;

        tensor
    }

    pub fn reshape<E: WGPUElement, const D1: usize, const D2: usize>(
        tensor: WGPUTensor<E, D1>,
        shape: Shape<D2>,
    ) -> WGPUTensor<E, D2> {
        // TODO: Not force standard layout all the time (improve performance).
        let tensor = Self::into_continuous(tensor);

        WGPUTensor::new(tensor.context, shape, tensor.buffer)
    }

    pub fn into_continuous<E: WGPUElement, const D: usize>(
        tensor: WGPUTensor<E, D>,
    ) -> WGPUTensor<E, D> {
        if tensor.is_continuous() {
            return tensor;
        }

        kernel_wgsl!(ContinuousRaw, "../template/continuous.wgsl");

        let buffer = tensor
            .context
            .create_buffer(tensor.shape.num_elements() * core::mem::size_of::<E>());
        let output = WGPUTensor::new(
            tensor.context.clone(),
            tensor.shape.clone(),
            Arc::new(buffer),
        );
        let info = build_binary_info(&tensor, &output);
        let info_buffer = tensor
            .context
            .create_buffer_with_data(bytemuck::cast_slice(&info));

        let kernel = tensor
            .context
            .compile::<KernelSettings<ContinuousRaw, E, i32, 256, 1, 1>>();

        tensor.context.execute(
            &WorkGroup::new(
                f32::ceil(output.shape.num_elements() as f32 / 256_f32) as u32,
                1,
                1,
            ),
            &kernel,
            &[&tensor.buffer, &output.buffer, &info_buffer],
        );

        output
    }
}
