use super::{record::AdaptorRecord, SimpleOptimizer};
use crate::{
    module::{ADModule, ModuleMapper, ParamId},
    optim::{GradientsParams, Optimizer},
    LearningRate,
};
use burn_tensor::{backend::ADBackend, Tensor};
use core::marker::PhantomData;
use hashbrown::HashMap;

/// Wrapper struct that adapts any [simple optimizer](SimpleOptimizer) into
/// an [optimizer](Optimizer).
pub struct OptimizerAdaptor<O, M, B>
where
    O: SimpleOptimizer<B::InnerBackend>,
    M: ADModule<B>,
    B: ADBackend,
{
    optim: O,
    records: HashMap<ParamId, AdaptorRecord<O, B::InnerBackend>>,
    module: PhantomData<M>,
}

impl<O, B, M> From<O> for OptimizerAdaptor<O, M, B>
where
    B: ADBackend,
    M: ADModule<B>,
    O: SimpleOptimizer<B::InnerBackend>,
{
    fn from(optim: O) -> Self {
        Self {
            optim,
            records: HashMap::new(),
            module: PhantomData::default(),
        }
    }
}

impl<O, B, M> Optimizer<M, B> for OptimizerAdaptor<O, M, B>
where
    B: ADBackend,
    M: ADModule<B>,
    O: SimpleOptimizer<B::InnerBackend>,
{
    type Record = HashMap<ParamId, AdaptorRecord<O, B::InnerBackend>>;

    fn step(&mut self, lr: LearningRate, module: M, mut grads: GradientsParams) -> M {
        let mut mapper =
            SimpleOptimizerMapper::<M, B, O>::new(&self.optim, &mut self.records, &mut grads, lr);
        module.map(&mut mapper)
    }

    fn to_record(&self) -> Self::Record {
        self.records.clone()
    }

    fn load_record(mut self, record: Self::Record) -> Self {
        self.records = record;
        self
    }
}

#[derive(new)]
struct SimpleOptimizerMapper<'a, M, B, O>
where
    M: ADModule<B>,
    B: ADBackend,
    O: SimpleOptimizer<B::InnerBackend>,
{
    optimizer: &'a O,
    records: &'a mut HashMap<ParamId, AdaptorRecord<O, B::InnerBackend>>,
    grads: &'a mut GradientsParams,
    lr: LearningRate,
    phatom: PhantomData<M>,
}

impl<'a, M, B, O> ModuleMapper<B> for SimpleOptimizerMapper<'a, M, B, O>
where
    M: ADModule<B>,
    B: ADBackend,
    O: SimpleOptimizer<B::InnerBackend>,
{
    fn map<const D: usize>(&mut self, id: &ParamId, tensor: Tensor<B, D>) -> Tensor<B, D> {
        let grad = self.grads.remove(id);

        if let Some(grad) = grad {
            let device = grad.device();
            let is_require_grad = tensor.is_require_grad();
            let (key, record) = self.records.remove_entry(id).unzip();

            let (tensor, state) = self.optimizer.step(
                self.lr,
                tensor.inner(),
                grad,
                record.map(|record| O::to_device(record.into_state(), &device)),
            );

            if let Some(state) = state {
                self.records.insert(
                    key.unwrap_or_else(|| id.clone()),
                    AdaptorRecord::from_state(state),
                );
            }

            let mut tensor = Tensor::from_inner(tensor);
            if is_require_grad {
                tensor = tensor.require_grad();
            }
            return tensor;
        }

        tensor
    }
}
