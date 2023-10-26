use crate::{
    grads::Gradients,
    ops::{unary, Backward, Ops, OpsKind},
    ADBackendDecorator,
};
use burn_tensor::{
    backend::Backend,
    ops::{ActivationOps, FloatTensor},
};

impl<B: Backend> ActivationOps<ADBackendDecorator<B>> for ADBackendDecorator<B> {
    fn gelu<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        #[derive(Debug)]
        struct Gelu<const D: usize>;

        impl<const D: usize, B: Backend> Backward<B, D, 1> for Gelu<D> {
            type State = B::TensorPrimitive<D>;

            fn backward(self, ops: Ops<Self::State, 1>, grads: &mut Gradients) {
                let input = ops.state;

                unary::<B, D, D, _>(ops.parents, ops.node, grads, |grad| {
                    B::gelu_backward(input, grad)
                });
            }
        }

        match Gelu::<D>.prepare([tensor.node], [tensor.graph]).stateful() {
            OpsKind::Tracked(prep) => {
                let output = B::gelu(tensor.primitive.clone());
                prep.finish(tensor.primitive, output)
            }
            OpsKind::UnTracked(prep) => prep.finish(B::gelu(tensor.primitive)),
        }
    }

    fn relu<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        #[derive(Debug)]
        struct Relu;

        impl<B: Backend, const D: usize> Backward<B, D, 1> for Relu {
            type State = B::TensorPrimitive<D>;

            fn backward(self, ops: Ops<Self::State, 1>, grads: &mut Gradients) {
                unary::<B, D, D, _>(ops.parents, ops.node, grads, |grad| {
                    B::relu_backward(ops.state, grad)
                });
            }
        }
        let output = B::relu(tensor.primitive);

        match Relu.prepare([tensor.node], [tensor.graph]).stateful() {
            OpsKind::Tracked(prep) => prep.finish(output.clone(), output),
            OpsKind::UnTracked(prep) => prep.finish(output),
        }
    }
}
