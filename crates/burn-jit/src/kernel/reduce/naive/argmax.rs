use super::base::ReduceDimNaive;
use crate::kernel::reduce::Argmax;
use cubecl::cube;
use cubecl::frontend::{Tensor, ABSOLUTE_POS};
use cubecl::prelude::{Cast, Numeric};

#[allow(clippy::extra_unused_type_parameters)]
#[cube]
impl<EI: Numeric> ReduceDimNaive<EI> for Argmax {
    type Accumulator = (f32, u32);

    fn initialize_naive() -> Self::Accumulator {
        // TODO: switch to using f32::NEG_INFINITY when it's supported: https://github.com/tracel-ai/cubecl/issues/68
        let a = 0.0_f32;
        let b = 100000000.0_f32;
        (a - b, 0u32)
    }

    fn inner_loop_naive(accumulator: &mut Self::Accumulator, current_value: EI, i: u32) {
        let (max, index) = accumulator;
        let val = f32::cast_from(current_value);
        if val > *max {
            *max = val;
            *index = i;
        }
    }

    fn assign_naive<EO: Numeric>(
        output: &mut Tensor<EO>,
        accumulator: Self::Accumulator,
        _shape_reduce_dim: u32,
    ) {
        let (_, index) = accumulator;
        output[ABSOLUTE_POS] = EO::cast_from(index);
    }
}
