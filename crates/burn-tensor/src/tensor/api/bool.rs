use crate::{backend::Backend, Bool, Data, Int, Tensor};

impl<B, const D: usize> Tensor<B, D, Bool>
where
    B: Backend,
{
    /// Create a boolean tensor from data on the given device.
    pub fn from_bool(data: Data<bool, D>, device: &B::Device) -> Self {
        Self::new(B::bool_from_data(data, device))
    }

    /// Convert the bool tensor into an int tensor.
    pub fn int(self) -> Tensor<B, D, Int> {
        Tensor::new(B::bool_into_int(self.primitive))
    }

    /// Convert the bool tensor into an float tensor.
    pub fn float(self) -> Tensor<B, D> {
        Tensor::new(B::bool_into_float(self.primitive))
    }

    /// Inverses boolean values.
    pub fn bool_not(self) -> Self {
        Tensor::new(B::bool_not(self.primitive))
    }

    // pub fn any(self) -> Tensor<B, 1, Bool> {
    //     Tensor::new(B::bool_any(self.primitive))
    // }
    //
    // pub fn any_dim(self, dim: usize) -> Tensor<B, D, Bool> {
    //     Tensor::new(B::bool_any_dim(self.primitive, dim))
    // }
    //
    // pub fn all(self) -> Tensor<B, 1, Bool> {
    //     Tensor::new(B::bool_all(self.primitive))
    // }
    //
    // pub fn all_dim(self, dim: usize) -> Tensor<B, D, Bool> {
    //     Tensor::new(B::bool_all_dim(self.primitive, dim))
    // }
}
