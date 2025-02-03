use super::Reshape;
use crate::{
    fusion::{on_write::settings::FuseSettings, JitFusionHandle},
    JitRuntime,
};
use burn_fusion::stream::Context;
use burn_tensor::repr::{TensorId, TensorStatus};
use std::marker::PhantomData;

use super::{HandleInput, LaunchPlan, PotentialInplace, RegisteredTensors};

/// Fetch and register [input handles](HandleInput) and itendify potential inputs that
/// can be used inplace.
pub struct InputsPlanner<'a, R: JitRuntime> {
    inputs: &'a RegisteredTensors,
    inputs_unhandled: &'a Vec<TensorId>,
    reshapes: &'a Vec<Reshape>,
    shape_ref: &'a Vec<usize>,
    settings: &'a FuseSettings,
    _r: PhantomData<R>,
}

impl<'a, R: JitRuntime> InputsPlanner<'a, R> {
    pub fn new(
        inputs: &'a RegisteredTensors,
        inputs_unhandled: &'a Vec<TensorId>,
        reshapes: &'a Vec<Reshape>,
        shape_ref: &'a Vec<usize>,
        settings: &'a FuseSettings,
    ) -> Self {
        Self {
            inputs,
            settings,
            inputs_unhandled,
            reshapes,
            shape_ref,
            _r: PhantomData,
        }
    }

    pub fn run(self, context: &mut Context<'_, JitFusionHandle<R>>, plan: &mut LaunchPlan<'a, R>) {
        for (i, (precision, tensor_relative)) in self.inputs.iter().enumerate() {
            let mut tensor_global = context.tensors.get(&tensor_relative.id).unwrap().clone();
            // Important to take the status of the relative graph and not
            // the global graph, since the status of the global graph
            // might be of a later operation on the same tensor id.
            let status = &tensor_relative.status;
            let mut handle = context.handles.get_handle(&tensor_global.id, status);

            if self.settings.inplace
                && status == &TensorStatus::ReadWrite
                && handle.handle.can_mut()
                && !self.inputs_unhandled.contains(&tensor_relative.id)
                && self
                    .reshapes
                    .iter()
                    .find(|r| r.reshaped == tensor_relative.id || r.original == tensor_relative.id)
                    .is_none()
                && self.shape_ref == &tensor_relative.shape
            {
                plan.potential_inplaces.push(PotentialInplace {
                    input_pos: i,
                    tensor_relative,
                    strides: handle.strides.clone(),
                });
            }

            if tensor_global.shape.len() < plan.rank {
                let num_elem: usize = tensor_global.shape.iter().product();
                for _ in 0..(plan.rank - tensor_global.shape.len()) {
                    tensor_global.shape.insert(0, 1);
                    handle.strides.insert(0, num_elem);
                }
            }

            plan.handle_inputs.push(HandleInput {
                precision,
                handle,
                relative_id: tensor_relative.id,
                global_id: tensor_global.id,
                global_shape: tensor_global.shape.clone(),
                vectorization: 1,
            });
            plan.global_inputs.push(tensor_global);
        }
    }
}
