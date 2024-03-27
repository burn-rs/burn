use crate::{
    codegen::{execute_dynamic, EagerHandle, WorkgroupLaunch},
    element::JitElement,
    ops::numeric::empty_device,
    tensor::JitTensor,
    Runtime,
};
use burn_tensor::Shape;

use super::AdaptivePool2dEagerKernel;

pub(crate) fn adaptive_avg_pool2d<R: Runtime, E: JitElement>(
    input: JitTensor<R, E, 4>,
    output_size: [usize; 2],
) -> JitTensor<R, E, 4> {
    let [batch_size, channels, _, _] = input.shape.dims;

    let output_shape = Shape::new([batch_size, channels, output_size[0], output_size[1]]);
    let output = empty_device(input.client.clone(), input.device.clone(), output_shape);

    let kernel = AdaptivePool2dEagerKernel::new();

    execute_dynamic::<R, AdaptivePool2dEagerKernel<R, E>, E>(
        &[EagerHandle::new(
            &input.handle,
            &input.strides,
            &input.shape.dims,
        )],
        &[EagerHandle::new(
            &output.handle,
            &output.strides,
            &output.shape.dims,
        )],
        None,
        kernel,
        WorkgroupLaunch::Output { pos: 0 },
        input.client,
    );

    output
}
