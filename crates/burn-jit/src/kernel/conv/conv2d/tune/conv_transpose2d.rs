use burn_tensor::{
    ops::{conv::calculate_conv_output_size, ConvTransposeOptions},
    ElementConversion, Shape,
};
use cubecl::{
    tune,
    tune::{local_tuner, tune_with, LocalTuner},
};

use crate::{
    kernel::{
        conv::{batches_per_run, conv_transpose2d_col2im, conv_transpose2d_direct},
        prng::random_uniform,
    },
    tensor::JitTensor,
    FloatElement, IntElement, JitAutotuneKey, JitRuntime, JitTuneId,
};

use super::ConvTranspose2dAutotuneKey;

/// Executes autotune on conv2d operations
pub fn conv_transpose2d_autotune<R: JitRuntime, E: FloatElement, I: IntElement>(
    input: JitTensor<R, E>,
    weights: JitTensor<R, E>,
    bias: Option<JitTensor<R, E>>,
    options: ConvTransposeOptions<2>,
) -> JitTensor<R, E> {
    let client = input.client.clone();

    static TUNER: LocalTuner<JitAutotuneKey, JitTuneId> = local_tuner!();

    TUNER.execute(
        &JitTuneId::new::<R>(&input.device),
        &client,
        Box::new(ConvTranspose2dOperations::<R, E, I>::new(
            input, weights, bias, options,
        )),
    )
}

#[tune(operations(conv_transpose2d_direct, conv_transpose2d_col2im), create_key = create_key, should_run = should_run)]
pub fn conv_transpose2d_operations<R: JitRuntime, E: FloatElement, I: IntElement>(
    key: JitAutotuneKey,
    input: JitTensor<R, E>,
    weights: JitTensor<R, E>,
    bias: Option<JitTensor<R, E>>,
    options: ConvTransposeOptions<2>,
) -> JitTensor<R, E> {
    let key = match key {
        JitAutotuneKey::ConvTranspose2d(key) => key,
        _ => unreachable!(),
    };
    let device = &input.device;

    let random_bounds: (E, E) = ((-1.0).elem::<E>(), (1.0).elem::<E>());
    let input_shape = Shape::new([key.batch_size, key.in_channels, key.height, key.width]);
    let input = random_uniform(input_shape, device, random_bounds.0, random_bounds.1);
    let c_per_grp = key.in_channels / key.groups;
    let [kernel_h, kernel_w] = key.kernel_size;
    let weight_shape = Shape::new([key.out_channels, c_per_grp, kernel_h, kernel_w]);
    let weights = random_uniform(weight_shape, device, random_bounds.0, random_bounds.1);
    let bias_shape = Shape::new([key.out_channels]);
    let bias = key
        .has_bias
        .then(|| random_uniform(bias_shape, device, random_bounds.0, random_bounds.1));
    tune_with!(input, weights, bias, options)
}

fn create_key<R: JitRuntime, E: FloatElement>(
    input: &JitTensor<R, E>,
    weights: &JitTensor<R, E>,
    bias: &Option<JitTensor<R, E>>,
    options: &ConvTransposeOptions<2>,
) -> JitAutotuneKey {
    let [batch_size, in_channels, height, width] = input.shape.dims();
    let [out_channels, _, kernel_h, kernel_w] = weights.shape.dims();
    let ConvTransposeOptions {
        stride,
        padding,
        dilation,
        groups,
        padding_out,
    } = options.clone();
    JitAutotuneKey::ConvTranspose2d(ConvTranspose2dAutotuneKey::new(
        [kernel_h, kernel_w],
        stride,
        padding,
        padding_out,
        dilation,
        groups,
        in_channels,
        out_channels,
        height,
        width,
        batch_size,
        bias.is_some(),
    ))
}

fn should_run<R: JitRuntime, F: FloatElement, I: IntElement>(
    op: &ConvTranspose2dOperations<R, F, I>,
    _key: &JitAutotuneKey,
    index: usize,
) -> bool {
    match index {
        // im2col
        1 => {
            let [batch_size, _, in_height, in_width] = op.input.shape.dims();
            let [_, _, kernel_h, kernel_w] = op.weights.shape.dims();

            let out_h = calculate_conv_output_size(
                kernel_h,
                op.options.stride[0],
                op.options.padding[0],
                op.options.dilation[0],
                in_height,
            );
            let out_w = calculate_conv_output_size(
                kernel_w,
                op.options.stride[1],
                op.options.padding[1],
                op.options.dilation[1],
                in_width,
            );

            batches_per_run(batch_size, out_h, out_w).is_some()
        }
        _ => true,
    }
}
