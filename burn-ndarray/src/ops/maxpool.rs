use crate::{
    element::FloatNdArrayElement, iter_par, ops::padding::apply_padding_4d, run_par,
    sharing::UnsafeSharedRef, tensor::NdArrayTensor,
};

use burn_tensor::ElementConversion;
use ndarray::Array4;

pub(crate) fn max_pool2d<E: FloatNdArrayElement>(
    x: NdArrayTensor<E, 4>,
    kernel_size: [usize; 2],
    stride: [usize; 2],
    padding: [usize; 2],
) -> NdArrayTensor<E, 4> {
    let [kernel_height, kernel_width] = kernel_size;
    let [padding_height, padding_width] = padding;
    let [stride_height, stride_width] = stride;
    let [batch_size, channels, x_height, x_width] = x.shape().dims;
    let inf = (-f32::INFINITY).elem::<E>();

    let out_height = ((x_height + 2 * padding_height - kernel_height) / stride_height) + 1;
    let out_width = ((x_width + 2 * padding_width - kernel_width) / stride_width) + 1;

    let x = apply_padding_4d(x, padding, inf).array;

    let mut output = Array4::from_elem((batch_size, channels, out_height, out_width), inf);
    let unsafe_shared_out = UnsafeSharedRef::new(&mut output);

    run_par!(|| {
        iter_par!(0, batch_size * channels).for_each(|k| unsafe {
            let b = k / channels;
            let c = k % channels;

            let output = unsafe_shared_out.get();

            for oh in 0..out_height {
                for ow in 0..out_width {
                    let mut max_val = inf;

                    for kh in 0..kernel_height {
                        let ih = oh * stride_height + kh;

                        for kw in 0..kernel_width {
                            let iw = ow * stride_width + kw;

                            let val = x[[b, c, ih, iw]];

                            if val > max_val {
                                max_val = val;
                            }
                        }
                    }

                    output[[b, c, oh, ow]] = max_val;
                }
            }
        })
    });

    NdArrayTensor::new(output.into_dyn().into_shared())
}

pub(crate) fn max_pool2d_with_indexes<E: FloatNdArrayElement>(
    x: NdArrayTensor<E, 4>,
    kernel_size: [usize; 2],
    stride: [usize; 2],
    padding: [usize; 2],
) -> (NdArrayTensor<E, 4>, NdArrayTensor<i64, 4>) {
    let [kernel_height, kernel_width] = kernel_size;
    let [padding_height, padding_width] = padding;
    let [stride_height, stride_width] = stride;
    let [batch_size, channels, x_height, x_width] = x.shape().dims;
    let inf = (-f32::INFINITY).elem::<E>();

    let out_height = ((x_height + 2 * padding_height - kernel_height) / stride_height) + 1;
    let out_width = ((x_width + 2 * padding_width - kernel_width) / stride_width) + 1;

    let x = apply_padding_4d(x, padding, inf).array;

    let mut output = Array4::from_elem((batch_size, channels, out_height, out_width), inf);
    let mut indexes = Array4::<i64>::zeros((batch_size, channels, out_height, out_width));

    let unsafe_shared_out = UnsafeSharedRef::new(&mut output);
    let unsafe_shared_indexes = UnsafeSharedRef::new(&mut indexes);

    run_par!(|| {
        iter_par!(0, batch_size * channels).for_each(|k| unsafe {
            let b = k / channels;
            let c = k % channels;

            let output = unsafe_shared_out.get();
            let indexes = unsafe_shared_indexes.get();

            for oh in 0..out_height {
                for ow in 0..out_width {
                    let mut max_val = inf;
                    let mut index = 0;

                    for kh in 0..kernel_height {
                        let ih = oh * stride_height + kh;

                        for kw in 0..kernel_width {
                            let iw = ow * stride_width + kw;
                            let val = x[[b, c, ih, iw]];

                            if val > max_val {
                                max_val = val;

                                let ih = ih as i64 - padding_height as i64;
                                let iw = iw as i64 - padding_width as i64;

                                index = ih * x_height as i64 + iw;
                            }
                        }
                    }

                    output[[b, c, oh, ow]] = max_val;
                    indexes[[b, c, oh, ow]] = index;
                }
            }
        })
    });

    let output = NdArrayTensor::new(output.into_dyn().into_shared());
    let indexes = NdArrayTensor::new(indexes.into_dyn().into_shared());

    (output, indexes)
}

pub(crate) fn max_pool2d_backward<E: FloatNdArrayElement>(
    x: NdArrayTensor<E, 4>,
    _kernel_size: [usize; 2],
    _stride: [usize; 2],
    _padding: [usize; 2],
    output_grad: NdArrayTensor<E, 4>,
    indexes: NdArrayTensor<i64, 4>,
) -> NdArrayTensor<E, 4> {
    let [_batch_size, _channels, height, width] = output_grad.shape().dims;
    let [batch_size, channels, height_x, width_x] = x.shape().dims;

    let output_grad = output_grad.array;
    let indexes = indexes.array;

    let mut output = Array4::zeros((batch_size, channels, height_x, width_x));

    let unsafe_shared_out = UnsafeSharedRef::new(&mut output);

    run_par!(|| {
        iter_par!(0, batch_size * channels).for_each(|k| unsafe {
            let b = k / channels;
            let c = k % channels;

            let output = unsafe_shared_out.get();

            for h in 0..height {
                for w in 0..width {
                    let index = indexes[[b, c, h, w]];
                    let grad = output_grad[[b, c, h, w]];

                    let index_h = index as usize / width_x;
                    let index_w = index as usize % width_x;

                    output[[b, c, index_h, index_w]] += grad;
                }
            }
        });
    });

    NdArrayTensor::new(output.into_dyn().into_shared())
}
