use crate::{client::FusionClient, stream::StreamId, Client, FusionBackend, FusionRuntime};
use burn_tensor::{
    quantization::{QTensorPrimitive, QuantizationScheme},
    repr::{TensorDescription, TensorId, TensorStatus},
    DType, Shape, TensorData, TensorMetadata,
};
use std::sync::Arc;

/// Tensor primitive for the [fusion backend](crate::FusionBackend) for all kind.
pub struct FusionTensor<R: FusionRuntime> {
    /// Tensor id.
    pub id: Arc<TensorId>,
    /// The shape of the tensor.
    pub shape: Vec<usize>,
    /// The [fusion client](FusionClient).
    pub client: Client<R>,
    /// The datatype of the tensor.
    pub dtype: DType,
    /// The current stream id this tensor is on.
    pub stream: StreamId,
    // Orphan means that a tensor is never converted into a description when it becomes `ReadWrite`.
    //
    // When a tensor is dropped and is still an orphan, we need to register it as such to avoid
    // memory leak. Otherwise, the cleanup is going to happen during a graph execution.
    pub(crate) is_orphan: bool,
}

impl<R: FusionRuntime> Clone for FusionTensor<R> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            shape: self.shape.clone(),
            client: self.client.clone(),
            dtype: self.dtype,
            is_orphan: self.is_orphan,
            stream: self.stream,
        }
    }
}

impl<R: FusionRuntime> core::fmt::Debug for FusionTensor<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "{{ id: {:?}, shape: {:?}, should_drop: {:?}, device: {:?} }}",
                self.id,
                self.shape,
                self.is_orphan,
                self.client.device().clone(),
            )
            .as_str(),
        )
    }
}

impl<R: FusionRuntime> TensorMetadata for FusionTensor<R> {
    fn dtype(&self) -> DType {
        self.dtype
    }

    fn shape(&self) -> Shape {
        Shape::from(self.shape.clone())
    }
}

impl<R: FusionRuntime> FusionTensor<R> {
    pub(crate) fn new(
        id: Arc<TensorId>,
        shape: Vec<usize>,
        dtype: DType,
        client: Client<R>,
        stream: StreamId,
    ) -> Self {
        Self {
            id,
            shape,
            client,
            dtype,
            is_orphan: true,
            stream,
        }
    }

    fn status(&self) -> TensorStatus {
        if Arc::strong_count(&self.id) <= 1 {
            TensorStatus::ReadWrite
        } else {
            TensorStatus::ReadOnly
        }
    }

    /// Description to be used when using an uninitialized tensor as output.
    pub fn to_description_out(&self) -> TensorDescription {
        TensorDescription {
            status: TensorStatus::NotInit,
            shape: self.shape.clone(),
            id: *self.id.as_ref(),
            dtype: self.dtype,
        }
    }

    /// Description to be used when using an initialized tensor used as input.
    pub fn into_description(mut self) -> TensorDescription {
        let status = self.status();
        let mut shape_out = Vec::new();
        core::mem::swap(&mut self.shape, &mut shape_out);

        if let TensorStatus::ReadWrite = status {
            self.is_orphan = false;
        }

        TensorDescription {
            status,
            shape: shape_out,
            id: *self.id.as_ref(),
            dtype: self.dtype,
        }
    }

    pub(crate) async fn into_data<B>(self) -> TensorData
    where
        B: FusionBackend<FusionRuntime = R>,
    {
        let id = self.stream;
        self.client
            .clone()
            .read_tensor_float::<B>(self.into_description(), id)
            .await
    }

    pub(crate) async fn q_into_data<B>(self) -> TensorData
    where
        B: FusionBackend<FusionRuntime = R>,
    {
        if let DType::QFloat(_scheme) = self.dtype {
            let id = self.stream;
            self.client
                .clone()
                .read_tensor_quantized::<B>(self.into_description(), id)
                .await
        } else {
            panic!("Expected quantized float dtype, got {:?}", self.dtype)
        }
    }

    pub(crate) async fn int_into_data<B>(self) -> TensorData
    where
        B: FusionBackend<FusionRuntime = R>,
    {
        let id = self.stream;
        self.client
            .clone()
            .read_tensor_int::<B>(self.into_description(), id)
            .await
    }

    pub(crate) async fn bool_into_data<B>(self) -> TensorData
    where
        B: FusionBackend<FusionRuntime = R>,
    {
        let id = self.stream;
        self.client
            .clone()
            .read_tensor_bool::<B>(self.into_description(), id)
            .await
    }
}

impl<R: FusionRuntime> Drop for FusionTensor<R> {
    fn drop(&mut self) {
        if !self.is_orphan {
            return;
        }

        match self.status() {
            TensorStatus::ReadWrite => {
                self.client.register_orphan(&self.id);
            }
            TensorStatus::ReadOnly => {}
            TensorStatus::NotInit => {}
        }
    }
}

impl<R: FusionRuntime> QTensorPrimitive for FusionTensor<R> {
    fn scheme(&self) -> &QuantizationScheme {
        if let DType::QFloat(scheme) = &self.dtype {
            scheme
        } else {
            panic!(
                "Quantization scheme is not valid for dtype {:?}",
                self.dtype,
            )
        }
    }
}
