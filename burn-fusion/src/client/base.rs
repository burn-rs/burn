use crate::{
    graph::{GraphExecution, TensorOpsDescription},
    FusedBackend, FusionTensor, TensorDescription, TensorId,
};
use burn_tensor::{
    ops::{FloatElem, IntElem},
    Data, Reader,
};

/// Define how to interact with the fusion server.
pub trait FusionClient: Send + Sync + Clone {
    /// The [fused backend](FusedBackend) associated type.
    type FusedBackend: FusedBackend;
    /// The [graph execution](GraphExecution) associated type.
    type GraphExecution: GraphExecution<Self::FusedBackend>;

    /// Create a new client for the given [handle device](FusedBackend::HandleDevice).
    fn new(device: <Self::FusedBackend as FusedBackend>::HandleDevice) -> Self;
    /// Register a new [tensor operation description](TensorOpsDescription).
    fn register(&self, ops: TensorOpsDescription<Self::FusedBackend>);
    /// Sync the computation.
    fn sync(&self);
    /// Get the current device used by all operations handled by this client.
    fn device<'a>(&'a self) -> &'a <Self::FusedBackend as FusedBackend>::HandleDevice;
    /// Create an empty tensor.
    fn create_tensor_empty(&self, shape: Vec<usize>) -> FusionTensor<Self>;
    /// Create a float tensor with the given values.
    fn create_tensor_float(
        &self,
        values: Vec<FloatElem<Self::FusedBackend>>,
        shape: Vec<usize>,
    ) -> FusionTensor<Self>;
    /// Create an integer tensor with the given values.
    fn create_tensor_int(
        &self,
        values: Vec<IntElem<Self::FusedBackend>>,
        shape: Vec<usize>,
    ) -> FusionTensor<Self>;
    /// Create a bool tensor with the given values.
    fn create_tensor_bool(&self, values: Vec<bool>, shape: Vec<usize>) -> FusionTensor<Self>;
    /// Read the values contained by a float tensor.
    fn read_tensor_float<const D: usize>(
        &self,
        tensor: TensorDescription,
    ) -> Reader<Data<FloatElem<Self::FusedBackend>, D>>;
    /// Read the values contained by an int tensor.
    fn read_tensor_int<const D: usize>(
        &self,
        tensor: TensorDescription,
    ) -> Reader<Data<IntElem<Self::FusedBackend>, D>>;
    /// Read the values contained by a bool tensor.
    fn read_tensor_bool<const D: usize>(&self, tensor: TensorDescription) -> Reader<Data<bool, D>>;
    /// Change the client of the given float tensor.
    fn change_client_float<const D: usize>(
        &self,
        tensor: TensorDescription,
        client: Self,
    ) -> FusionTensor<Self>;
    /// Change the client of the given int tensor.
    fn change_client_int<const D: usize>(
        &self,
        tensor: TensorDescription,
        client: Self,
    ) -> FusionTensor<Self>;
    /// Change the client of the given bool tensor.
    fn change_client_bool<const D: usize>(
        &self,
        tensor: TensorDescription,
        client: Self,
    ) -> FusionTensor<Self>;
    /// Drop the tensor with the given [tensor id](TensorId).
    fn drop_tensor(&self, id: &TensorId);
}
