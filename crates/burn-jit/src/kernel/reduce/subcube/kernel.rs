use cubecl::{prelude::*, CubeCount, CubeDim};

use crate::{tensor::JitTensor, JitElement, JitRuntime};

use super::base::ReduceDimSubcube;

#[cube(launch_unchecked)]
pub fn reduce_dim_subcube_kernel<
    RD: ReduceDimSubcube<EIn, EOut>,
    EIn: JitElement,
    EOut: JitElement,
>(
    input: &Tensor<EIn>,
    output: &mut Tensor<EOut>,
    #[comptime] dim: u32,
    #[comptime] smem_size: u32,
    #[comptime] elems_per_thread: u32,
    #[comptime] divisible_shape: bool,
) {
    let stride_reduce_dim_input = input.stride(dim);
    let shape_reduce_dim_input = input.shape(dim);

    let reduce_group_id = CUBE_POS;
    let warp_count = CUBE_DIM / SUBCUBE_DIM;
    let warp_id = UNIT_POS / SUBCUBE_DIM;

    let mut shared_memory = RD::init_shared(smem_size);

    let mut index_offset = 0;

    for i in 0..input.rank() {
        let num_block = reduce_group_id / output.stride(i) % output.shape(i);
        index_offset += num_block * input.stride(i);
    }

    let mut value = RD::init_value();

    for i in 0..elems_per_thread {
        let nth = i * CUBE_DIM + UNIT_POS;
        let current_pos = nth * stride_reduce_dim_input + index_offset;

        #[allow(clippy::collapsible_else_if)]
        if divisible_shape {
            let next = RD::read_value(input, current_pos, nth);
            RD::update_value(&mut value, next);
        } else {
            if nth < shape_reduce_dim_input {
                let next = RD::read_value(input, current_pos, nth);
                RD::update_value(&mut value, next);
            }
        }
    }

    RD::reduce_subcube(&mut shared_memory, warp_id, value);

    sync_units();

    let mut n_warps = warp_count;

    while n_warps > 1 {
        n_warps /= 2;

        if UNIT_POS == 0 && warp_id < n_warps {
            let read_pos = n_warps + warp_id;
            let mut value = RD::read_from_shared(&shared_memory, read_pos);
            let next = RD::read_from_shared(&shared_memory, warp_id);
            RD::update_value(&mut value, next);
            RD::write_to_shared(&mut shared_memory, warp_id, value);
        }

        sync_units();
    }

    if UNIT_POS == 0 {
        RD::store(
            &shared_memory,
            output,
            reduce_group_id,
            shape_reduce_dim_input,
        );
    }
}

/// Executes the shared memory kernel for reduce dim
pub fn reduce_dim_subcube<
    RD: ReduceDimSubcube<EI, EO>,
    R: JitRuntime,
    EI: JitElement,
    EO: JitElement,
>(
    input: JitTensor<R, EI>,
    output: JitTensor<R, EO>,
    dim: usize,
) -> JitTensor<R, EO> {
    let num_elems_output = output.shape.num_elements();
    let cube_dim = CubeDim::default();
    let cube_count_x = f32::ceil(f32::sqrt(num_elems_output as f32));
    let cube_count_y = f32::ceil(num_elems_output as f32 / cube_count_x);
    let cube_count = CubeCount::Static(cube_count_x as u32, cube_count_y as u32, 1);

    let reduce_group_size = input.shape.dims[dim];
    let n_invocation_per_cube = cube_dim.num_elems();
    let elems_per_thread =
        f32::ceil(reduce_group_size as f32 / n_invocation_per_cube as f32) as u32;

    let divisible_shape = n_invocation_per_cube * elems_per_thread == reduce_group_size as u32;

    unsafe {
        reduce_dim_subcube_kernel::launch_unchecked::<RD, EI, EO, R>(
            &input.client,
            cube_count,
            cube_dim,
            input.as_tensor_arg(1),
            output.as_tensor_arg(1),
            dim as u32,
            cube_dim.num_elems(),
            elems_per_thread,
            divisible_shape,
        )
    };

    output
}
