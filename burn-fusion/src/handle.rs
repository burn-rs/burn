use crate::{FusedBackend, TensorDescription, TensorId, TensorStatus};
use burn_tensor::{
    ops::{FloatElem, IntElem},
    Data, Shape,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct HandleContainer<B: FusedBackend> {
    handles: HashMap<TensorId, Handle<B>>,
    counter: u64,
    pub device: B::Device,
}

enum Handle<B: FusedBackend> {
    Empty,
    DataFloat(Vec<FloatElem<B>>),
    DataInt(Vec<IntElem<B>>),
    Existing(B::Handle),
}

impl<B: FusedBackend> HandleContainer<B> {
    pub fn new(device_handle: B::HandleDevice) -> Self {
        Self {
            handles: HashMap::new(),
            counter: 0,
            device: device_handle.clone().into(),
        }
    }

    pub fn get_float_tensor<const D: usize>(
        &mut self,
        tensor: &TensorDescription,
    ) -> B::TensorPrimitive<D> {
        let (id, handle) = self.handles.remove_entry(&tensor.id).unwrap();

        if let Handle::Existing(handle) = handle {
            match tensor.status {
                TensorStatus::ReadOnly => {
                    self.handles.insert(id, Handle::Existing(handle.clone()));
                    return B::float_tensor(handle, Shape::from(tensor.shape.clone()));
                }
                TensorStatus::ReadWrite => {
                    return B::float_tensor(handle, Shape::from(tensor.shape.clone()));
                }
            }
        }

        let output = match handle {
            Handle::Empty => B::empty(Shape::from(tensor.shape.clone()), &self.device),
            Handle::DataFloat(values) => B::from_data(
                Data::new(values, Shape::from(tensor.shape.clone())),
                &self.device,
            ),
            Handle::Existing(handle) => B::float_tensor(handle, Shape::from(tensor.shape.clone())),
            Handle::DataInt(_) => panic!("From int unsupported when getting float tensor."),
        };

        if let TensorStatus::ReadOnly = tensor.status {
            self.handles
                .insert(id, Handle::Existing(B::float_tensor_handle(output.clone())));
        }

        output
    }

    pub fn get_int_tensor<const D: usize>(
        &mut self,
        tensor: &TensorDescription,
    ) -> B::IntTensorPrimitive<D> {
        let (id, handle) = self.handles.remove_entry(&tensor.id).unwrap();

        if let Handle::Existing(handle) = handle {
            match tensor.status {
                TensorStatus::ReadOnly => {
                    self.handles.insert(id, Handle::Existing(handle.clone()));
                    return B::int_tensor(handle, Shape::from(tensor.shape.clone()));
                }
                TensorStatus::ReadWrite => {
                    return B::int_tensor(handle, Shape::from(tensor.shape.clone()));
                }
            }
        }

        let output = match handle {
            Handle::Empty => B::int_empty(Shape::from(tensor.shape.clone()), &self.device),
            Handle::DataInt(values) => B::int_from_data(
                Data::new(values, Shape::from(tensor.shape.clone())),
                &self.device,
            ),
            Handle::Existing(handle) => B::int_tensor(handle, Shape::from(tensor.shape.clone())),
            Handle::DataFloat(_) => panic!("From float unsupported when getting int tensor."),
        };

        if let TensorStatus::ReadOnly = tensor.status {
            self.handles
                .insert(id, Handle::Existing(B::int_tensor_handle(output.clone())));
        }

        output
    }

    pub fn register_float_tensor<const D: usize>(
        &mut self,
        id: &TensorId,
        tensor: B::TensorPrimitive<D>,
    ) {
        let handle = B::float_tensor_handle(tensor);
        self.handles.insert(id.clone(), Handle::Existing(handle));
    }

    pub fn register_int_tensor<const D: usize>(
        &mut self,
        id: &TensorId,
        tensor: B::IntTensorPrimitive<D>,
    ) {
        let handle = B::int_tensor_handle(tensor);
        self.handles.insert(id.clone(), Handle::Existing(handle));
    }

    pub fn create_emtpy(&mut self) -> Arc<TensorId> {
        let id = TensorId::new(self.counter);
        self.counter += 1;
        self.handles.insert(id.clone(), Handle::Empty);

        Arc::new(id)
    }

    pub fn create_float(&mut self, values: Vec<FloatElem<B>>) -> Arc<TensorId> {
        let id = TensorId::new(self.counter);
        self.counter += 1;
        println!("create float {:?} - count {:?}", id, self.counter);
        self.handles.insert(id.clone(), Handle::DataFloat(values));

        Arc::new(id)
    }

    pub fn cleanup(&mut self, tensor: &TensorDescription) {
        match tensor.status {
            TensorStatus::ReadOnly => (),
            TensorStatus::ReadWrite => {
                self.handles.remove(&tensor.id);
            }
        }
    }
}
