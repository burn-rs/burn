use burn_fusion::OptimizationBuilder;

use crate::{
    fusion::{
        on_write::{builder::FuseOnWriteBuilder, ir::ElemwisePrecision, settings::FuseSettings},
        CubeOptimization,
    },
    CubeRuntime,
};

use super::optimization::ElemwiseOptimization;

/// Fused element wise operations that are normally memory bound.
pub(crate) struct ElementWiseBuilder<R: CubeRuntime> {
    builder: FuseOnWriteBuilder,
    device: R::Device,
}

impl<R: CubeRuntime> ElementWiseBuilder<R> {
    pub fn new(device: R::Device, bool_precision: ElemwisePrecision) -> Self {
        let client = R::client(&device);
        let props = client.properties();
        let max_bindings = props.hardware_properties().max_bindings;

        Self {
            builder: FuseOnWriteBuilder::new(
                max_bindings,
                bool_precision,
                FuseSettings {
                    broadcast: true,
                    output_shape_updates: true,
                    mix_vectorization: true,
                    inplace: true,
                },
            ),
            device,
        }
    }
}

impl<R: CubeRuntime> OptimizationBuilder<CubeOptimization<R>> for ElementWiseBuilder<R> {
    fn register(&mut self, operation: &burn_ir::OperationIr) {
        self.builder.register(operation);
    }

    fn build(&self) -> CubeOptimization<R> {
        let client = R::client(&self.device);
        let trace = self.builder.build();
        let elementwise =
            ElemwiseOptimization::<R>::new(trace, client, self.device.clone(), self.len());

        CubeOptimization::ElementWise(elementwise)
    }

    fn reset(&mut self) {
        self.builder.reset()
    }

    fn status(&self) -> burn_fusion::OptimizationStatus {
        self.builder.status()
    }

    fn properties(&self) -> burn_fusion::OptimizationProperties {
        self.builder.properties()
    }

    fn len(&self) -> usize {
        self.builder.len()
    }
}
